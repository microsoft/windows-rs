#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IVpnAppId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnAppIdFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannel4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannel5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannel6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannelActivityEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannelActivityStateChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannelConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannelConfiguration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnChannelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomCheckBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomComboBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomEditBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomErrorBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomPrompt(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomPromptBooleanInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomPromptElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomPromptOptionSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomPromptText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomPromptTextInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnCustomTextBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnDomainNameAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnDomainNameInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnDomainNameInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnDomainNameInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnForegroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnForegroundActivationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnInterfaceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnInterfaceIdFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnManagementAgent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnNamespaceAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnNamespaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnNamespaceInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnNativeProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnNativeProfile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPacketBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPacketBuffer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPacketBuffer3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPacketBufferFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPacketBufferList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPacketBufferList2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPickedCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPlugIn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPlugInProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnPlugInProfile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnRouteAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnRouteFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnSystemHealth(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnTrafficFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnTrafficFilterAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVpnTrafficFilterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnAppId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnAppIdType(pub i32);
impl VpnAppIdType {
    pub const PackageFamilyName: VpnAppIdType = VpnAppIdType(0i32);
    pub const FullyQualifiedBinaryName: VpnAppIdType = VpnAppIdType(1i32);
    pub const FilePath: VpnAppIdType = VpnAppIdType(2i32);
}
#[repr(transparent)]
pub struct VpnAuthenticationMethod(pub i32);
impl VpnAuthenticationMethod {
    pub const Mschapv2: VpnAuthenticationMethod = VpnAuthenticationMethod(0i32);
    pub const Eap: VpnAuthenticationMethod = VpnAuthenticationMethod(1i32);
    pub const Certificate: VpnAuthenticationMethod = VpnAuthenticationMethod(2i32);
    pub const PresharedKey: VpnAuthenticationMethod = VpnAuthenticationMethod(3i32);
}
#[repr(transparent)]
pub struct VpnChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelActivityEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelActivityEventType(pub i32);
impl VpnChannelActivityEventType {
    pub const Idle: VpnChannelActivityEventType = VpnChannelActivityEventType(0i32);
    pub const Active: VpnChannelActivityEventType = VpnChannelActivityEventType(1i32);
}
#[repr(transparent)]
pub struct VpnChannelActivityStateChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelRequestCredentialsOptions(pub u32);
impl VpnChannelRequestCredentialsOptions {
    pub const None: VpnChannelRequestCredentialsOptions = VpnChannelRequestCredentialsOptions(0u32);
    pub const Retrying: VpnChannelRequestCredentialsOptions = VpnChannelRequestCredentialsOptions(1u32);
    pub const UseForSingleSignIn: VpnChannelRequestCredentialsOptions = VpnChannelRequestCredentialsOptions(2u32);
}
#[repr(transparent)]
pub struct VpnCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCredentialType(pub i32);
impl VpnCredentialType {
    pub const UsernamePassword: VpnCredentialType = VpnCredentialType(0i32);
    pub const UsernameOtpPin: VpnCredentialType = VpnCredentialType(1i32);
    pub const UsernamePasswordAndPin: VpnCredentialType = VpnCredentialType(2i32);
    pub const UsernamePasswordChange: VpnCredentialType = VpnCredentialType(3i32);
    pub const SmartCard: VpnCredentialType = VpnCredentialType(4i32);
    pub const ProtectedCertificate: VpnCredentialType = VpnCredentialType(5i32);
    pub const UnProtectedCertificate: VpnCredentialType = VpnCredentialType(6i32);
}
#[repr(transparent)]
pub struct VpnCustomCheckBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomComboBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomEditBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomErrorBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomPromptBooleanInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomPromptOptionSelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomPromptText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomPromptTextInput(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCustomTextBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnDataPathType(pub i32);
impl VpnDataPathType {
    pub const Send: VpnDataPathType = VpnDataPathType(0i32);
    pub const Receive: VpnDataPathType = VpnDataPathType(1i32);
}
#[repr(transparent)]
pub struct VpnDomainNameAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnDomainNameInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnDomainNameType(pub i32);
impl VpnDomainNameType {
    pub const Suffix: VpnDomainNameType = VpnDomainNameType(0i32);
    pub const FullyQualified: VpnDomainNameType = VpnDomainNameType(1i32);
    pub const Reserved: VpnDomainNameType = VpnDomainNameType(65535i32);
}
#[repr(transparent)]
pub struct VpnForegroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnForegroundActivationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnIPProtocol(pub i32);
impl VpnIPProtocol {
    pub const None: VpnIPProtocol = VpnIPProtocol(0i32);
    pub const Tcp: VpnIPProtocol = VpnIPProtocol(6i32);
    pub const Udp: VpnIPProtocol = VpnIPProtocol(17i32);
    pub const Icmp: VpnIPProtocol = VpnIPProtocol(1i32);
    pub const Ipv6Icmp: VpnIPProtocol = VpnIPProtocol(58i32);
    pub const Igmp: VpnIPProtocol = VpnIPProtocol(2i32);
    pub const Pgm: VpnIPProtocol = VpnIPProtocol(113i32);
}
#[repr(transparent)]
pub struct VpnInterfaceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnManagementAgent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnManagementConnectionStatus(pub i32);
impl VpnManagementConnectionStatus {
    pub const Disconnected: VpnManagementConnectionStatus = VpnManagementConnectionStatus(0i32);
    pub const Disconnecting: VpnManagementConnectionStatus = VpnManagementConnectionStatus(1i32);
    pub const Connected: VpnManagementConnectionStatus = VpnManagementConnectionStatus(2i32);
    pub const Connecting: VpnManagementConnectionStatus = VpnManagementConnectionStatus(3i32);
}
#[repr(transparent)]
pub struct VpnManagementErrorStatus(pub i32);
impl VpnManagementErrorStatus {
    pub const Ok: VpnManagementErrorStatus = VpnManagementErrorStatus(0i32);
    pub const Other: VpnManagementErrorStatus = VpnManagementErrorStatus(1i32);
    pub const InvalidXmlSyntax: VpnManagementErrorStatus = VpnManagementErrorStatus(2i32);
    pub const ProfileNameTooLong: VpnManagementErrorStatus = VpnManagementErrorStatus(3i32);
    pub const ProfileInvalidAppId: VpnManagementErrorStatus = VpnManagementErrorStatus(4i32);
    pub const AccessDenied: VpnManagementErrorStatus = VpnManagementErrorStatus(5i32);
    pub const CannotFindProfile: VpnManagementErrorStatus = VpnManagementErrorStatus(6i32);
    pub const AlreadyDisconnecting: VpnManagementErrorStatus = VpnManagementErrorStatus(7i32);
    pub const AlreadyConnected: VpnManagementErrorStatus = VpnManagementErrorStatus(8i32);
    pub const GeneralAuthenticationFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(9i32);
    pub const EapFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(10i32);
    pub const SmartCardFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(11i32);
    pub const CertificateFailure: VpnManagementErrorStatus = VpnManagementErrorStatus(12i32);
    pub const ServerConfiguration: VpnManagementErrorStatus = VpnManagementErrorStatus(13i32);
    pub const NoConnection: VpnManagementErrorStatus = VpnManagementErrorStatus(14i32);
    pub const ServerConnection: VpnManagementErrorStatus = VpnManagementErrorStatus(15i32);
    pub const UserNamePassword: VpnManagementErrorStatus = VpnManagementErrorStatus(16i32);
    pub const DnsNotResolvable: VpnManagementErrorStatus = VpnManagementErrorStatus(17i32);
    pub const InvalidIP: VpnManagementErrorStatus = VpnManagementErrorStatus(18i32);
}
#[repr(transparent)]
pub struct VpnNamespaceAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnNamespaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnNativeProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnNativeProtocolType(pub i32);
impl VpnNativeProtocolType {
    pub const Pptp: VpnNativeProtocolType = VpnNativeProtocolType(0i32);
    pub const L2tp: VpnNativeProtocolType = VpnNativeProtocolType(1i32);
    pub const IpsecIkev2: VpnNativeProtocolType = VpnNativeProtocolType(2i32);
}
#[repr(transparent)]
pub struct VpnPacketBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnPacketBufferList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnPacketBufferStatus(pub i32);
impl VpnPacketBufferStatus {
    pub const Ok: VpnPacketBufferStatus = VpnPacketBufferStatus(0i32);
    pub const InvalidBufferSize: VpnPacketBufferStatus = VpnPacketBufferStatus(1i32);
}
#[repr(transparent)]
pub struct VpnPickedCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnPlugInProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnRouteAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnRoutingPolicyType(pub i32);
impl VpnRoutingPolicyType {
    pub const SplitRouting: VpnRoutingPolicyType = VpnRoutingPolicyType(0i32);
    pub const ForceAllTrafficOverVpn: VpnRoutingPolicyType = VpnRoutingPolicyType(1i32);
}
#[repr(transparent)]
pub struct VpnSystemHealth(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnTrafficFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnTrafficFilterAssignment(pub *mut ::core::ffi::c_void);
