#[cfg(feature = "implement_exclusive")]
pub trait IVpnAppIdImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<VpnAppIdType>;
    fn SetType(&self, value: VpnAppIdType) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnAppIdFactoryImpl: Sized {
    fn Create(&self, r#type: VpnAppIdType, value: &::windows::core::HSTRING) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelImpl: Sized {
    fn AssociateTransport(&self, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Start(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, routescope: &::core::option::Option<VpnRouteAssignment>, namespacescope: &::core::option::Option<VpnNamespaceAssignment>, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn RequestCredentials(&self, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<VpnPickedCredential>;
    fn RequestVpnPacketBuffer(&self, r#type: VpnDataPathType, vpnpacketbuffer: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn LogDiagnosticMessage(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Configuration(&self) -> ::windows::core::Result<VpnChannelConfiguration>;
    fn ActivityChange(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivityChange(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetPlugInContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn PlugInContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SystemHealth(&self) -> ::windows::core::Result<VpnSystemHealth>;
    fn RequestCustomPrompt(&self, customprompt: &::core::option::Option<super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt>>) -> ::windows::core::Result<()>;
    fn SetErrorMessage(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetAllowedSslTlsVersions(&self, tunneltransport: &::core::option::Option<::windows::core::IInspectable>, usetls12: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel2Impl: Sized {
    fn StartWithMainTransport(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assigneddomainname: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartExistingTransports(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assigneddomainname: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows::core::Result<()>;
    fn ActivityStateChange(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivityStateChange(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetVpnSendPacketBuffer(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn GetVpnReceivePacketBuffer(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn RequestCustomPromptAsync(&self, custompromptelement: &::core::option::Option<super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestCredentialsWithCertificateAsync(&self, credtype: VpnCredentialType, credoptions: u32, certificate: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn RequestCredentialsWithOptionsAsync(&self, credtype: VpnCredentialType, credoptions: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn RequestCredentialsSimpleAsync(&self, credtype: VpnCredentialType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn TerminateConnection(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartWithTrafficFilter(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assignednamespace: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, assignedtrafficfilters: &::core::option::Option<VpnTrafficFilterAssignment>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel4Impl: Sized {
    fn AddAndAssociateTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartWithMultipleTransports(&self, assignedclientipv4addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, assignedclientipv6addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assignednamespace: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, transports: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, assignedtrafficfilters: &::core::option::Option<VpnTrafficFilterAssignment>) -> ::windows::core::Result<()>;
    fn ReplaceAndAssociateTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartReconnectingTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetSlotTypeForTransportContext(&self, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::Sockets::ControlChannelTriggerStatus>;
    fn CurrentRequestTransportContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel5Impl: Sized {
    fn AppendVpnReceivePacketBuffer(&self, decapsulatedpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn AppendVpnSendPacketBuffer(&self, encapsulatedpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn FlushVpnReceivePacketBuffers(&self) -> ::windows::core::Result<()>;
    fn FlushVpnSendPacketBuffers(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel6Impl: Sized {
    fn ActivateForeground(&self, packagerelativeappid: &::windows::core::HSTRING, sharedcontext: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelActivityEventArgsImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelActivityStateChangedArgsImpl: Sized {
    fn ActivityState(&self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelConfigurationImpl: Sized {
    fn ServerServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServerHostNameList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
    fn CustomField(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelConfiguration2Impl: Sized {
    fn ServerUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
}
pub trait IVpnChannelStaticsImpl: Sized {
    fn ProcessEventAsync(&self, thirdpartyplugin: &::core::option::Option<::windows::core::IInspectable>, event: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
pub trait IVpnCredentialImpl: Sized {
    fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CertificateCredential(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomCheckBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetInitialCheckState(&self, value: bool) -> ::windows::core::Result<()>;
    fn InitialCheckState(&self) -> ::windows::core::Result<bool>;
    fn Checked(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomComboBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetOptionsText(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn OptionsText(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Selected(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomEditBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetDefaultText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DefaultText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNoEcho(&self, value: bool) -> ::windows::core::Result<()>;
    fn NoEcho(&self) -> ::windows::core::Result<bool>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomErrorBoxImpl: Sized + IVpnCustomPromptImpl {}
pub trait IVpnCustomPromptImpl: Sized {
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&self) -> ::windows::core::Result<bool>;
    fn SetBordered(&self, value: bool) -> ::windows::core::Result<()>;
    fn Bordered(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptBooleanInputImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetInitialValue(&self, value: bool) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<bool>;
}
pub trait IVpnCustomPromptElementImpl: Sized {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&self) -> ::windows::core::Result<bool>;
    fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()>;
    fn Emphasized(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptOptionSelectorImpl: Sized + IVpnCustomPromptElementImpl {
    fn Options(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SelectedIndex(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptTextImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptTextInputImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIsTextHidden(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTextHidden(&self) -> ::windows::core::Result<bool>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomTextBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnDomainNameAssignmentImpl: Sized {
    fn DomainNameList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn SetProxyAutoConfigurationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAutoConfigurationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnDomainNameInfoImpl: Sized {
    fn SetDomainName(&self, value: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn DomainName(&self) -> ::windows::core::Result<super::HostName>;
    fn SetDomainNameType(&self, value: VpnDomainNameType) -> ::windows::core::Result<()>;
    fn DomainNameType(&self) -> ::windows::core::Result<VpnDomainNameType>;
    fn DnsServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
    fn WebProxyServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnDomainNameInfo2Impl: Sized {
    fn WebProxyUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
}
pub trait IVpnDomainNameInfoFactoryImpl: Sized {
    fn CreateVpnDomainNameInfo(&self, name: &::windows::core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>) -> ::windows::core::Result<VpnDomainNameInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnForegroundActivatedEventArgsImpl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SharedContext(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn ActivationOperation(&self) -> ::windows::core::Result<VpnForegroundActivationOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnForegroundActivationOperationImpl: Sized {
    fn Complete(&self, result: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnInterfaceIdImpl: Sized {
    fn GetAddressInfo(&self, id: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
pub trait IVpnInterfaceIdFactoryImpl: Sized {
    fn CreateVpnInterfaceId(&self, address: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VpnInterfaceId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnManagementAgentImpl: Sized {
    fn AddProfileFromXmlAsync(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn AddProfileFromObjectAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn UpdateProfileFromXmlAsync(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn UpdateProfileFromObjectAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn GetProfilesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IVpnProfile>>>;
    fn DeleteProfileAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn ConnectProfileAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn ConnectProfileWithPasswordCredentialAsync(&self, profile: &::core::option::Option<IVpnProfile>, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn DisconnectProfileAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNamespaceAssignmentImpl: Sized {
    fn SetNamespaceList(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>) -> ::windows::core::Result<()>;
    fn NamespaceList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>;
    fn SetProxyAutoConfigUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAutoConfigUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNamespaceInfoImpl: Sized {
    fn SetNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Namespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDnsServers(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<()>;
    fn DnsServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
    fn SetWebProxyServers(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<()>;
    fn WebProxyServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
}
pub trait IVpnNamespaceInfoFactoryImpl: Sized {
    fn CreateVpnNamespaceInfo(&self, name: &::windows::core::HSTRING, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<VpnNamespaceInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNativeProfileImpl: Sized + IVpnProfileImpl {
    fn Servers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RoutingPolicyType(&self) -> ::windows::core::Result<VpnRoutingPolicyType>;
    fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()>;
    fn NativeProtocolType(&self) -> ::windows::core::Result<VpnNativeProtocolType>;
    fn SetNativeProtocolType(&self, value: VpnNativeProtocolType) -> ::windows::core::Result<()>;
    fn UserAuthenticationMethod(&self) -> ::windows::core::Result<VpnAuthenticationMethod>;
    fn SetUserAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()>;
    fn TunnelAuthenticationMethod(&self) -> ::windows::core::Result<VpnAuthenticationMethod>;
    fn SetTunnelAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()>;
    fn EapConfiguration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEapConfiguration(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNativeProfile2Impl: Sized {
    fn RequireVpnClientAppUI(&self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBufferImpl: Sized {
    fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
    fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<VpnPacketBufferStatus>;
    fn SetTransportAffinity(&self, value: u32) -> ::windows::core::Result<()>;
    fn TransportAffinity(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBuffer2Impl: Sized {
    fn AppId(&self) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBuffer3Impl: Sized {
    fn SetTransportContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TransportContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
pub trait IVpnPacketBufferFactoryImpl: Sized {
    fn CreateVpnPacketBuffer(&self, parentbuffer: &::core::option::Option<VpnPacketBuffer>, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPacketBufferListImpl: Sized + IIterableImpl<VpnPacketBuffer> {
    fn Append(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn AddAtBegin(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn RemoveAtBegin(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<VpnPacketBufferStatus>;
    fn Size(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPacketBufferList2Impl: Sized + IIterableImpl<VpnPacketBuffer> {
    fn AddLeadingPacket(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveLeadingPacket(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn AddTrailingPacket(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveTrailingPacket(&self) -> ::windows::core::Result<VpnPacketBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPickedCredentialImpl: Sized {
    fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
pub trait IVpnPlugInImpl: Sized {
    fn Connect(&self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn Disconnect(&self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn GetKeepAlivePayload(&self, channel: &::core::option::Option<VpnChannel>, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn Encapsulate(&self, channel: &::core::option::Option<VpnChannel>, packets: &::core::option::Option<VpnPacketBufferList>, encapulatedpackets: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
    fn Decapsulate(&self, channel: &::core::option::Option<VpnChannel>, encapbuffer: &::core::option::Option<VpnPacketBuffer>, decapsulatedpackets: &::core::option::Option<VpnPacketBufferList>, controlpacketstosend: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPlugInProfileImpl: Sized + IVpnProfileImpl {
    fn ServerUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn CustomConfiguration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCustomConfiguration(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VpnPluginPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVpnPluginPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPlugInProfile2Impl: Sized + IVpnProfileImpl {
    fn RequireVpnClientAppUI(&self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
pub trait IVpnProfileImpl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>>;
    fn Routes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn DomainNameInfoList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn TrafficFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn RememberCredentials(&self) -> ::windows::core::Result<bool>;
    fn SetRememberCredentials(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysOn(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysOn(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnRouteImpl: Sized {
    fn SetAddress(&self, value: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn Address(&self) -> ::windows::core::Result<super::HostName>;
    fn SetPrefixSize(&self, value: u8) -> ::windows::core::Result<()>;
    fn PrefixSize(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnRouteAssignmentImpl: Sized {
    fn SetIpv4InclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn SetIpv6InclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn Ipv4InclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn Ipv6InclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn SetIpv4ExclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn SetIpv6ExclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn Ipv4ExclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn Ipv6ExclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn SetExcludeLocalSubnets(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExcludeLocalSubnets(&self) -> ::windows::core::Result<bool>;
}
pub trait IVpnRouteFactoryImpl: Sized {
    fn CreateVpnRoute(&self, address: &::core::option::Option<super::HostName>, prefixsize: u8) -> ::windows::core::Result<VpnRoute>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnSystemHealthImpl: Sized {
    fn StatementOfHealth(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnTrafficFilterImpl: Sized {
    fn AppId(&self) -> ::windows::core::Result<VpnAppId>;
    fn SetAppId(&self, value: &::core::option::Option<VpnAppId>) -> ::windows::core::Result<()>;
    fn AppClaims(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Protocol(&self) -> ::windows::core::Result<VpnIPProtocol>;
    fn SetProtocol(&self, value: VpnIPProtocol) -> ::windows::core::Result<()>;
    fn LocalPortRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RemotePortRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn LocalAddressRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RemoteAddressRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RoutingPolicyType(&self) -> ::windows::core::Result<VpnRoutingPolicyType>;
    fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnTrafficFilterAssignmentImpl: Sized {
    fn TrafficFilterList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn AllowOutbound(&self) -> ::windows::core::Result<bool>;
    fn SetAllowOutbound(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowInbound(&self) -> ::windows::core::Result<bool>;
    fn SetAllowInbound(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnTrafficFilterFactoryImpl: Sized {
    fn Create(&self, appid: &::core::option::Option<VpnAppId>) -> ::windows::core::Result<VpnTrafficFilter>;
}
