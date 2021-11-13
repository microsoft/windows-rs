#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IVpnAppId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnAppId {}
impl ::core::clone::Clone for IVpnAppId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnAppIdFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnAppIdFactory {}
impl ::core::clone::Clone for IVpnAppIdFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannel {}
impl ::core::clone::Clone for IVpnChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannel2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannel2 {}
impl ::core::clone::Clone for IVpnChannel2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannel4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannel4 {}
impl ::core::clone::Clone for IVpnChannel4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannel5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannel5 {}
impl ::core::clone::Clone for IVpnChannel5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannel6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannel6 {}
impl ::core::clone::Clone for IVpnChannel6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannelActivityEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannelActivityEventArgs {}
impl ::core::clone::Clone for IVpnChannelActivityEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannelActivityStateChangedArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannelActivityStateChangedArgs {}
impl ::core::clone::Clone for IVpnChannelActivityStateChangedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannelConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannelConfiguration {}
impl ::core::clone::Clone for IVpnChannelConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannelConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannelConfiguration2 {}
impl ::core::clone::Clone for IVpnChannelConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnChannelStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnChannelStatics {}
impl ::core::clone::Clone for IVpnChannelStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCredential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCredential {}
impl ::core::clone::Clone for IVpnCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomCheckBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomCheckBox {}
impl ::core::clone::Clone for IVpnCustomCheckBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomComboBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomComboBox {}
impl ::core::clone::Clone for IVpnCustomComboBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomEditBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomEditBox {}
impl ::core::clone::Clone for IVpnCustomEditBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomErrorBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomErrorBox {}
impl ::core::clone::Clone for IVpnCustomErrorBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomPrompt(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomPrompt {}
impl ::core::clone::Clone for IVpnCustomPrompt {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomPromptBooleanInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomPromptBooleanInput {}
impl ::core::clone::Clone for IVpnCustomPromptBooleanInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomPromptElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomPromptElement {}
impl ::core::clone::Clone for IVpnCustomPromptElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomPromptOptionSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomPromptOptionSelector {}
impl ::core::clone::Clone for IVpnCustomPromptOptionSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomPromptText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomPromptText {}
impl ::core::clone::Clone for IVpnCustomPromptText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomPromptTextInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomPromptTextInput {}
impl ::core::clone::Clone for IVpnCustomPromptTextInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnCustomTextBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnCustomTextBox {}
impl ::core::clone::Clone for IVpnCustomTextBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnDomainNameAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnDomainNameAssignment {}
impl ::core::clone::Clone for IVpnDomainNameAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnDomainNameInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnDomainNameInfo {}
impl ::core::clone::Clone for IVpnDomainNameInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnDomainNameInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnDomainNameInfo2 {}
impl ::core::clone::Clone for IVpnDomainNameInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnDomainNameInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnDomainNameInfoFactory {}
impl ::core::clone::Clone for IVpnDomainNameInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnForegroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnForegroundActivatedEventArgs {}
impl ::core::clone::Clone for IVpnForegroundActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnForegroundActivationOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnForegroundActivationOperation {}
impl ::core::clone::Clone for IVpnForegroundActivationOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnInterfaceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnInterfaceId {}
impl ::core::clone::Clone for IVpnInterfaceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnInterfaceIdFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnInterfaceIdFactory {}
impl ::core::clone::Clone for IVpnInterfaceIdFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnManagementAgent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnManagementAgent {}
impl ::core::clone::Clone for IVpnManagementAgent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnNamespaceAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnNamespaceAssignment {}
impl ::core::clone::Clone for IVpnNamespaceAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnNamespaceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnNamespaceInfo {}
impl ::core::clone::Clone for IVpnNamespaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnNamespaceInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnNamespaceInfoFactory {}
impl ::core::clone::Clone for IVpnNamespaceInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnNativeProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnNativeProfile {}
impl ::core::clone::Clone for IVpnNativeProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnNativeProfile2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnNativeProfile2 {}
impl ::core::clone::Clone for IVpnNativeProfile2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPacketBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPacketBuffer {}
impl ::core::clone::Clone for IVpnPacketBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPacketBuffer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPacketBuffer2 {}
impl ::core::clone::Clone for IVpnPacketBuffer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPacketBuffer3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPacketBuffer3 {}
impl ::core::clone::Clone for IVpnPacketBuffer3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPacketBufferFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPacketBufferFactory {}
impl ::core::clone::Clone for IVpnPacketBufferFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPacketBufferList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPacketBufferList {}
impl ::core::clone::Clone for IVpnPacketBufferList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPacketBufferList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPacketBufferList2 {}
impl ::core::clone::Clone for IVpnPacketBufferList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPickedCredential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPickedCredential {}
impl ::core::clone::Clone for IVpnPickedCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPlugIn(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPlugIn {}
impl ::core::clone::Clone for IVpnPlugIn {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPlugInProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPlugInProfile {}
impl ::core::clone::Clone for IVpnPlugInProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnPlugInProfile2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnPlugInProfile2 {}
impl ::core::clone::Clone for IVpnPlugInProfile2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnProfile {}
impl ::core::clone::Clone for IVpnProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnRoute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnRoute {}
impl ::core::clone::Clone for IVpnRoute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnRouteAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnRouteAssignment {}
impl ::core::clone::Clone for IVpnRouteAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnRouteFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnRouteFactory {}
impl ::core::clone::Clone for IVpnRouteFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnSystemHealth(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnSystemHealth {}
impl ::core::clone::Clone for IVpnSystemHealth {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnTrafficFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnTrafficFilter {}
impl ::core::clone::Clone for IVpnTrafficFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnTrafficFilterAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnTrafficFilterAssignment {}
impl ::core::clone::Clone for IVpnTrafficFilterAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVpnTrafficFilterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVpnTrafficFilterFactory {}
impl ::core::clone::Clone for IVpnTrafficFilterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnAppId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnAppId {}
impl ::core::clone::Clone for VpnAppId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnAppIdType(pub i32);
impl VpnAppIdType {
    pub const PackageFamilyName: Self = Self(0i32);
    pub const FullyQualifiedBinaryName: Self = Self(1i32);
    pub const FilePath: Self = Self(2i32);
}
impl ::core::marker::Copy for VpnAppIdType {}
impl ::core::clone::Clone for VpnAppIdType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnAuthenticationMethod(pub i32);
impl VpnAuthenticationMethod {
    pub const Mschapv2: Self = Self(0i32);
    pub const Eap: Self = Self(1i32);
    pub const Certificate: Self = Self(2i32);
    pub const PresharedKey: Self = Self(3i32);
}
impl ::core::marker::Copy for VpnAuthenticationMethod {}
impl ::core::clone::Clone for VpnAuthenticationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnChannel {}
impl ::core::clone::Clone for VpnChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnChannelActivityEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnChannelActivityEventArgs {}
impl ::core::clone::Clone for VpnChannelActivityEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnChannelActivityEventType(pub i32);
impl VpnChannelActivityEventType {
    pub const Idle: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnChannelActivityEventType {}
impl ::core::clone::Clone for VpnChannelActivityEventType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnChannelActivityStateChangedArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnChannelActivityStateChangedArgs {}
impl ::core::clone::Clone for VpnChannelActivityStateChangedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnChannelConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnChannelConfiguration {}
impl ::core::clone::Clone for VpnChannelConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnChannelRequestCredentialsOptions(pub u32);
impl VpnChannelRequestCredentialsOptions {
    pub const None: Self = Self(0u32);
    pub const Retrying: Self = Self(1u32);
    pub const UseForSingleSignIn: Self = Self(2u32);
}
impl ::core::marker::Copy for VpnChannelRequestCredentialsOptions {}
impl ::core::clone::Clone for VpnChannelRequestCredentialsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCredential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCredential {}
impl ::core::clone::Clone for VpnCredential {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for VpnCredentialType {}
impl ::core::clone::Clone for VpnCredentialType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomCheckBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomCheckBox {}
impl ::core::clone::Clone for VpnCustomCheckBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomComboBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomComboBox {}
impl ::core::clone::Clone for VpnCustomComboBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomEditBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomEditBox {}
impl ::core::clone::Clone for VpnCustomEditBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomErrorBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomErrorBox {}
impl ::core::clone::Clone for VpnCustomErrorBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomPromptBooleanInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomPromptBooleanInput {}
impl ::core::clone::Clone for VpnCustomPromptBooleanInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomPromptOptionSelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomPromptOptionSelector {}
impl ::core::clone::Clone for VpnCustomPromptOptionSelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomPromptText(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomPromptText {}
impl ::core::clone::Clone for VpnCustomPromptText {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomPromptTextInput(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomPromptTextInput {}
impl ::core::clone::Clone for VpnCustomPromptTextInput {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnCustomTextBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnCustomTextBox {}
impl ::core::clone::Clone for VpnCustomTextBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnDataPathType(pub i32);
impl VpnDataPathType {
    pub const Send: Self = Self(0i32);
    pub const Receive: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnDataPathType {}
impl ::core::clone::Clone for VpnDataPathType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnDomainNameAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnDomainNameAssignment {}
impl ::core::clone::Clone for VpnDomainNameAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnDomainNameInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnDomainNameInfo {}
impl ::core::clone::Clone for VpnDomainNameInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnDomainNameType(pub i32);
impl VpnDomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
    pub const Reserved: Self = Self(65535i32);
}
impl ::core::marker::Copy for VpnDomainNameType {}
impl ::core::clone::Clone for VpnDomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnForegroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnForegroundActivatedEventArgs {}
impl ::core::clone::Clone for VpnForegroundActivatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnForegroundActivationOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnForegroundActivationOperation {}
impl ::core::clone::Clone for VpnForegroundActivationOperation {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for VpnIPProtocol {}
impl ::core::clone::Clone for VpnIPProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnInterfaceId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnInterfaceId {}
impl ::core::clone::Clone for VpnInterfaceId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnManagementAgent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnManagementAgent {}
impl ::core::clone::Clone for VpnManagementAgent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnManagementConnectionStatus(pub i32);
impl VpnManagementConnectionStatus {
    pub const Disconnected: Self = Self(0i32);
    pub const Disconnecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Connecting: Self = Self(3i32);
}
impl ::core::marker::Copy for VpnManagementConnectionStatus {}
impl ::core::clone::Clone for VpnManagementConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for VpnManagementErrorStatus {}
impl ::core::clone::Clone for VpnManagementErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnNamespaceAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnNamespaceAssignment {}
impl ::core::clone::Clone for VpnNamespaceAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnNamespaceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnNamespaceInfo {}
impl ::core::clone::Clone for VpnNamespaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnNativeProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnNativeProfile {}
impl ::core::clone::Clone for VpnNativeProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnNativeProtocolType(pub i32);
impl VpnNativeProtocolType {
    pub const Pptp: Self = Self(0i32);
    pub const L2tp: Self = Self(1i32);
    pub const IpsecIkev2: Self = Self(2i32);
}
impl ::core::marker::Copy for VpnNativeProtocolType {}
impl ::core::clone::Clone for VpnNativeProtocolType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnPacketBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnPacketBuffer {}
impl ::core::clone::Clone for VpnPacketBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnPacketBufferList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnPacketBufferList {}
impl ::core::clone::Clone for VpnPacketBufferList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnPacketBufferStatus(pub i32);
impl VpnPacketBufferStatus {
    pub const Ok: Self = Self(0i32);
    pub const InvalidBufferSize: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnPacketBufferStatus {}
impl ::core::clone::Clone for VpnPacketBufferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnPickedCredential(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnPickedCredential {}
impl ::core::clone::Clone for VpnPickedCredential {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnPlugInProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnPlugInProfile {}
impl ::core::clone::Clone for VpnPlugInProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnRoute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnRoute {}
impl ::core::clone::Clone for VpnRoute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnRouteAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnRouteAssignment {}
impl ::core::clone::Clone for VpnRouteAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnRoutingPolicyType(pub i32);
impl VpnRoutingPolicyType {
    pub const SplitRouting: Self = Self(0i32);
    pub const ForceAllTrafficOverVpn: Self = Self(1i32);
}
impl ::core::marker::Copy for VpnRoutingPolicyType {}
impl ::core::clone::Clone for VpnRoutingPolicyType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnSystemHealth(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnSystemHealth {}
impl ::core::clone::Clone for VpnSystemHealth {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnTrafficFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnTrafficFilter {}
impl ::core::clone::Clone for VpnTrafficFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VpnTrafficFilterAssignment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VpnTrafficFilterAssignment {}
impl ::core::clone::Clone for VpnTrafficFilterAssignment {
    fn clone(&self) -> Self {
        *self
    }
}
