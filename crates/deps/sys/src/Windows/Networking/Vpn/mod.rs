#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct VpnAppIdType(i32);
#[repr(C)]
pub struct VpnAuthenticationMethod(i32);
#[repr(transparent)]
pub struct VpnChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelActivityEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnChannelActivityEventType(i32);
#[repr(transparent)]
pub struct VpnChannelActivityStateChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnChannelConfiguration(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnChannelRequestCredentialsOptions(i32);
#[repr(transparent)]
pub struct VpnCredential(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnCredentialType(i32);
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
#[repr(C)]
pub struct VpnDataPathType(i32);
#[repr(transparent)]
pub struct VpnDomainNameAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnDomainNameInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnDomainNameType(i32);
#[repr(transparent)]
pub struct VpnForegroundActivatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnForegroundActivationOperation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnIPProtocol(i32);
#[repr(transparent)]
pub struct VpnInterfaceId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnManagementAgent(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnManagementConnectionStatus(i32);
#[repr(C)]
pub struct VpnManagementErrorStatus(i32);
#[repr(transparent)]
pub struct VpnNamespaceAssignment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnNamespaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnNativeProfile(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnNativeProtocolType(i32);
#[repr(transparent)]
pub struct VpnPacketBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnPacketBufferList(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnPacketBufferStatus(i32);
#[repr(transparent)]
pub struct VpnPickedCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnPlugInProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnRouteAssignment(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct VpnRoutingPolicyType(i32);
#[repr(transparent)]
pub struct VpnSystemHealth(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnTrafficFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VpnTrafficFilterAssignment(pub *mut ::core::ffi::c_void);
