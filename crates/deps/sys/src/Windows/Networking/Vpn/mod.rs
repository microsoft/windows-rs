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
    pub const PackageFamilyName: Self = Self(0i32);
    pub const FullyQualifiedBinaryName: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
}
#[repr(transparent)]
pub struct VpnAuthenticationMethod(pub i32);
impl VpnAuthenticationMethod {
    pub const Mschapv2: Self = Self(0i32);
    pub const Eap: Self = Self(1i32);
    pub const Certificate: Self = Self(2i32);
    pub const PresharedKey: Self = Self(3i32);
}
#[repr(transparent)]
pub struct VpnChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelActivityEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelActivityEventType(pub i32);
impl VpnChannelActivityEventType {
    pub const Idle: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
#[repr(transparent)]
pub struct VpnChannelActivityStateChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelRequestCredentialsOptions(pub u32);
impl VpnChannelRequestCredentialsOptions {
    pub const None: Self = Self(0u32);
    pub const Retrying: Self = Self(1u32);
    pub const UseForSingleSignIn: Self = Self(2u32);
}
#[repr(transparent)]
pub struct VpnCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnCredentialType(pub i32);
impl VpnCredentialType {
    pub const UsernamePassword: Self = Self(0i32);
    pub const UsernameOtpPin: Self = Self(1i32);
    pub const UsernamePasswordAndPin: Self = Self(2i32);
    pub const UsernamePasswordChange: Self = Self(3i32);
    pub const SmartCard: Self = Self(4i32);
    pub const ProtectedCertificate: Self = Self(5i32);
    pub const UnProtectedCertificate: Self = Self(6i32);
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
    pub const Send: Self = Self(0i32);
    pub const Receive: Self = Self(1i32);
}
#[repr(transparent)]
pub struct VpnDomainNameAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnDomainNameInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnDomainNameType(pub i32);
impl VpnDomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
    pub const Reserved: Self = Self(65535i32);
}
#[repr(transparent)]
pub struct VpnForegroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnForegroundActivationOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnIPProtocol(pub i32);
impl VpnIPProtocol {
    pub const None: Self = Self(0i32);
    pub const Tcp: Self = Self(6i32);
    pub const Udp: Self = Self(17i32);
    pub const Icmp: Self = Self(1i32);
    pub const Ipv6Icmp: Self = Self(58i32);
    pub const Igmp: Self = Self(2i32);
    pub const Pgm: Self = Self(113i32);
}
#[repr(transparent)]
pub struct VpnInterfaceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnManagementAgent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnManagementConnectionStatus(pub i32);
impl VpnManagementConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Disconnecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Connecting: Self = Self(3i32);
}
#[repr(transparent)]
pub struct VpnManagementErrorStatus(pub i32);
impl VpnManagementErrorStatus {
    pub const Ok: Self = Self(0i32);
    pub const Other: Self = Self(1i32);
    pub const InvalidXmlSyntax: Self = Self(2i32);
    pub const ProfileNameTooLong: Self = Self(3i32);
    pub const ProfileInvalidAppId: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
    pub const CannotFindProfile: Self = Self(6i32);
    pub const AlreadyDisconnecting: Self = Self(7i32);
    pub const AlreadyConnected: Self = Self(8i32);
    pub const GeneralAuthenticationFailure: Self = Self(9i32);
    pub const EapFailure: Self = Self(10i32);
    pub const SmartCardFailure: Self = Self(11i32);
    pub const CertificateFailure: Self = Self(12i32);
    pub const ServerConfiguration: Self = Self(13i32);
    pub const NoConnection: Self = Self(14i32);
    pub const ServerConnection: Self = Self(15i32);
    pub const UserNamePassword: Self = Self(16i32);
    pub const DnsNotResolvable: Self = Self(17i32);
    pub const InvalidIP: Self = Self(18i32);
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
    pub const Pptp: Self = Self(0i32);
    pub const L2tp: Self = Self(1i32);
    pub const IpsecIkev2: Self = Self(2i32);
}
#[repr(transparent)]
pub struct VpnPacketBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnPacketBufferList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnPacketBufferStatus(pub i32);
impl VpnPacketBufferStatus {
    pub const Ok: Self = Self(0i32);
    pub const InvalidBufferSize: Self = Self(1i32);
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
    pub const SplitRouting: Self = Self(0i32);
    pub const ForceAllTrafficOverVpn: Self = Self(1i32);
}
#[repr(transparent)]
pub struct VpnSystemHealth(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnTrafficFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnTrafficFilterAssignment(pub *mut ::core::ffi::c_void);
