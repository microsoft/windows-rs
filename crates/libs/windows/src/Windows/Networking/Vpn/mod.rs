#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnAppId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnAppId {
    type Vtable = IVpnAppId_Vtbl;
}
impl ::core::clone::Clone for IVpnAppId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnAppId {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b06a635_5c58_41d9_94a7_bfbcf1d8ca54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnAppId_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnAppIdType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnAppIdType) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnAppIdFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnAppIdFactory {
    type Vtable = IVpnAppIdFactory_Vtbl;
}
impl ::core::clone::Clone for IVpnAppIdFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnAppIdFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46adfd2a_0aab_4fdb_821d_d3ddc919788b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnAppIdFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VpnAppIdType, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannel {
    type Vtable = IVpnChannel_Vtbl;
}
impl ::core::clone::Clone for IVpnChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ac78d07_d1a8_4303_a091_c8d2e0915bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AssociateTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, routescope: *mut ::core::ffi::c_void, namespacescope: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub RequestCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    RequestCredentials: usize,
    pub RequestVpnPacketBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VpnDataPathType, vpnpacketbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LogDiagnosticMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActivityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActivityChange: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivityChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivityChange: usize,
    pub SetPlugInContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PlugInContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SystemHealth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestCustomPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customprompt: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestCustomPrompt: usize,
    pub SetErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAllowedSslTlsVersions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tunneltransport: *mut ::core::ffi::c_void, usetls12: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannel2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannel2 {
    type Vtable = IVpnChannel2_Vtbl;
}
impl ::core::clone::Clone for IVpnChannel2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannel2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2255d165_993b_4629_ad60_f1c3f3537f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithMainTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assigneddomainname: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithMainTransport: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartExistingTransports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assigneddomainname: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartExistingTransports: usize,
    #[cfg(feature = "Foundation")]
    pub ActivityStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActivityStateChange: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivityStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivityStateChange: usize,
    pub GetVpnSendPacketBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVpnReceivePacketBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestCustomPromptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, custompromptelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestCustomPromptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub RequestCredentialsWithCertificateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, certificate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Certificates")))]
    RequestCredentialsWithCertificateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCredentialsWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCredentialsWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCredentialsSimpleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCredentialsSimpleAsync: usize,
    pub TerminateConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithTrafficFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assignedclientipv4list: *mut ::core::ffi::c_void, assignedclientipv6list: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assignednamespace: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void, assignedtrafficfilters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithTrafficFilter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannel4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannel4 {
    type Vtable = IVpnChannel4_Vtbl;
}
impl ::core::clone::Clone for IVpnChannel4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannel4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7266ede_2937_419d_9570_486aebb81803);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddAndAssociateTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithMultipleTransports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, assignedclientipv4addresses: *mut ::core::ffi::c_void, assignedclientipv6addresses: *mut ::core::ffi::c_void, vpninterfaceid: *mut ::core::ffi::c_void, assignedroutes: *mut ::core::ffi::c_void, assignednamespace: *mut ::core::ffi::c_void, mtusize: u32, maxframesize: u32, reserved: bool, transports: *mut ::core::ffi::c_void, assignedtrafficfilters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithMultipleTransports: usize,
    pub ReplaceAndAssociateTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartReconnectingTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub GetSlotTypeForTransportContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut super::Sockets::ControlChannelTriggerStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    GetSlotTypeForTransportContext: usize,
    pub CurrentRequestTransportContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannel5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannel5 {
    type Vtable = IVpnChannel5_Vtbl;
}
impl ::core::clone::Clone for IVpnChannel5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannel5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde7a0992_8384_4fbc_882c_1fd23124cd3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppendVpnReceivePacketBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, decapsulatedpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppendVpnSendPacketBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encapsulatedpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FlushVpnReceivePacketBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FlushVpnSendPacketBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannel6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannel6 {
    type Vtable = IVpnChannel6_Vtbl;
}
impl ::core::clone::Clone for IVpnChannel6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannel6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55843696_bd63_49c5_abca_5da77885551a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannel6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivateForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagerelativeappid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sharedcontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivateForeground: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannelActivityEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannelActivityEventArgs {
    type Vtable = IVpnChannelActivityEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVpnChannelActivityEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannelActivityEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa36c88f2_afdc_4775_855d_d4ac0a35fc55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelActivityEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannelActivityStateChangedArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannelActivityStateChangedArgs {
    type Vtable = IVpnChannelActivityStateChangedArgs_Vtbl;
}
impl ::core::clone::Clone for IVpnChannelActivityStateChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannelActivityStateChangedArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d750565_fdc0_4bbe_a23b_45fffc6d97a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelActivityStateChangedArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ActivityState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannelConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannelConfiguration {
    type Vtable = IVpnChannelConfiguration_Vtbl;
}
impl ::core::clone::Clone for IVpnChannelConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannelConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e2ddca2_2012_4fe4_b179_8c652c6d107e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServerServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerHostNameList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerHostNameList: usize,
    pub CustomField: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnChannelConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnChannelConfiguration2 {
    type Vtable = IVpnChannelConfiguration2_Vtbl;
}
impl ::core::clone::Clone for IVpnChannelConfiguration2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannelConfiguration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf30b574c_7824_471c_a118_63dbc93ae4c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerUris: usize,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnChannelStatics(::windows_core::IUnknown);
impl IVpnChannelStatics {
    pub fn ProcessEventAsync<P0, P1>(&self, thirdpartyplugin: P0, event: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ProcessEventAsync)(::windows_core::Interface::as_raw(this), thirdpartyplugin.into_param().abi(), event.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnChannelStatics, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnChannelStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnChannelStatics {}
impl ::core::fmt::Debug for IVpnChannelStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnChannelStatics").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnChannelStatics {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{88eb062d-e818-4ffd-98a6-363e3736c95d}");
}
unsafe impl ::windows_core::Interface for IVpnChannelStatics {
    type Vtable = IVpnChannelStatics_Vtbl;
}
impl ::core::clone::Clone for IVpnChannelStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnChannelStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88eb062d_e818_4ffd_98a6_363e3736c95d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnChannelStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProcessEventAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnCredential(::windows_core::IUnknown);
impl IVpnCredential {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PasskeyCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn CertificateCredential(&self) -> ::windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CertificateCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AdditionalPin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalPin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldPasswordCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnCredential, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnCredential {}
impl ::core::fmt::Debug for IVpnCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnCredential").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnCredential {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{b7e78af3-a46d-404b-8729-1832522853ac}");
}
unsafe impl ::windows_core::Interface for IVpnCredential {
    type Vtable = IVpnCredential_Vtbl;
}
impl ::core::clone::Clone for IVpnCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCredential {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7e78af3_a46d_404b_8729_1832522853ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCredential_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub PasskeyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasskeyCredential: usize,
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub CertificateCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Cryptography_Certificates"))]
    CertificateCredential: usize,
    pub AdditionalPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub OldPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OldPasswordCredential: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomCheckBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomCheckBox {
    type Vtable = IVpnCustomCheckBox_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomCheckBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomCheckBox {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43878753_03c5_4e61_93d7_a957714c4282);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomCheckBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetInitialCheckState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InitialCheckState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Checked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomComboBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomComboBox {
    type Vtable = IVpnCustomComboBox_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomComboBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomComboBox {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a24158e_dba1_4c6f_8270_dcf3c9761c4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomComboBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetOptionsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetOptionsText: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OptionsText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OptionsText: usize,
    pub Selected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomEditBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomEditBox {
    type Vtable = IVpnCustomEditBox_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomEditBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomEditBox {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3002d9a0_cfbf_4c0b_8f3c_66f503c20b39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomEditBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDefaultText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DefaultText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNoEcho: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub NoEcho: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomErrorBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomErrorBox {
    type Vtable = IVpnCustomErrorBox_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomErrorBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomErrorBox {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ec4efb2_c942_42af_b223_588b48328721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomErrorBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnCustomPrompt(::windows_core::IUnknown);
impl IVpnCustomPrompt {
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBordered)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bordered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnCustomPrompt, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnCustomPrompt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnCustomPrompt {}
impl ::core::fmt::Debug for IVpnCustomPrompt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnCustomPrompt").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnCustomPrompt {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{9b2ebe7b-87d5-433c-b4f6-eee6aa68a244}");
}
unsafe impl ::windows_core::Interface for IVpnCustomPrompt {
    type Vtable = IVpnCustomPrompt_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomPrompt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomPrompt {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b2ebe7b_87d5_433c_b4f6_eee6aa68a244);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPrompt_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCompulsory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Compulsory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetBordered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Bordered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomPromptBooleanInput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomPromptBooleanInput {
    type Vtable = IVpnCustomPromptBooleanInput_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomPromptBooleanInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomPromptBooleanInput {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4c9a69e_ff47_4527_9f27_a49292019979);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptBooleanInput_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetInitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub InitialValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnCustomPromptElement(::windows_core::IUnknown);
impl IVpnCustomPromptElement {
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEmphasized)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emphasized)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnCustomPromptElement, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnCustomPromptElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnCustomPromptElement {}
impl ::core::fmt::Debug for IVpnCustomPromptElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnCustomPromptElement").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnCustomPromptElement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{73bd5638-6f04-404d-93dd-50a44924a38b}");
}
unsafe impl ::windows_core::Interface for IVpnCustomPromptElement {
    type Vtable = IVpnCustomPromptElement_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomPromptElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomPromptElement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73bd5638_6f04_404d_93dd_50a44924a38b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptElement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCompulsory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Compulsory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEmphasized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Emphasized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomPromptOptionSelector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomPromptOptionSelector {
    type Vtable = IVpnCustomPromptOptionSelector_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomPromptOptionSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomPromptOptionSelector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b8f34d9_8ec1_4e95_9a4e_7ba64d38f330);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptOptionSelector_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomPromptText(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomPromptText {
    type Vtable = IVpnCustomPromptText_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomPromptText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomPromptText {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bc8bdee_3a42_49a3_abdd_07b2edea752d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptText_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomPromptTextInput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomPromptTextInput {
    type Vtable = IVpnCustomPromptTextInput_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomPromptTextInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomPromptTextInput {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9da9c75_913c_47d5_88ba_48fc48930235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomPromptTextInput_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetPlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PlaceholderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetIsTextHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsTextHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnCustomTextBox(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnCustomTextBox {
    type Vtable = IVpnCustomTextBox_Vtbl;
}
impl ::core::clone::Clone for IVpnCustomTextBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnCustomTextBox {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdaa4c3ca_8f23_4d36_91f1_76d937827942);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnCustomTextBox_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnDomainNameAssignment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnDomainNameAssignment {
    type Vtable = IVpnDomainNameAssignment_Vtbl;
}
impl ::core::clone::Clone for IVpnDomainNameAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnDomainNameAssignment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4135b141_ccdb_49b5_9401_039a8ae767e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameAssignment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DomainNameList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DomainNameList: usize,
    #[cfg(feature = "Foundation")]
    pub SetProxyAutoConfigurationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProxyAutoConfigurationUri: usize,
    #[cfg(feature = "Foundation")]
    pub ProxyAutoConfigurationUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProxyAutoConfigurationUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnDomainNameInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnDomainNameInfo {
    type Vtable = IVpnDomainNameInfo_Vtbl;
}
impl ::core::clone::Clone for IVpnDomainNameInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnDomainNameInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad2eb82f_ea8e_4f7a_843e_1a87e32e1b9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDomainNameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnDomainNameType) -> ::windows_core::HRESULT,
    pub DomainNameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnDomainNameType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsServers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyServers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyServers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnDomainNameInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnDomainNameInfo2 {
    type Vtable = IVpnDomainNameInfo2_Vtbl;
}
impl ::core::clone::Clone for IVpnDomainNameInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnDomainNameInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab871151_6c53_4828_9883_d886de104407);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyUris: usize,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnDomainNameInfoFactory(::windows_core::IUnknown);
impl IVpnDomainNameInfoFactory {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnDomainNameInfo<P0, P1>(&self, name: &::windows_core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: P0, proxyserverlist: P1) -> ::windows_core::Result<VpnDomainNameInfo>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::HostName>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnDomainNameInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), nametype, dnsserverlist.try_into_param()?.abi(), proxyserverlist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnDomainNameInfoFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnDomainNameInfoFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnDomainNameInfoFactory {}
impl ::core::fmt::Debug for IVpnDomainNameInfoFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnDomainNameInfoFactory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnDomainNameInfoFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2507bb75-028f-4688-8d3a-c4531df37da8}");
}
unsafe impl ::windows_core::Interface for IVpnDomainNameInfoFactory {
    type Vtable = IVpnDomainNameInfoFactory_Vtbl;
}
impl ::core::clone::Clone for IVpnDomainNameInfoFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnDomainNameInfoFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2507bb75_028f_4688_8d3a_c4531df37da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnDomainNameInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateVpnDomainNameInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: *mut ::core::ffi::c_void, proxyserverlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateVpnDomainNameInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnForegroundActivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnForegroundActivatedEventArgs {
    type Vtable = IVpnForegroundActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVpnForegroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnForegroundActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85b465b0_cadb_4d70_ac92_543a24dc9ebc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnForegroundActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SharedContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SharedContext: usize,
    pub ActivationOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnForegroundActivationOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnForegroundActivationOperation {
    type Vtable = IVpnForegroundActivationOperation_Vtbl;
}
impl ::core::clone::Clone for IVpnForegroundActivationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnForegroundActivationOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e010d57_f17a_4bd5_9b6d_f984f1297d3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnForegroundActivationOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Complete: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnInterfaceId(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnInterfaceId {
    type Vtable = IVpnInterfaceId_Vtbl;
}
impl ::core::clone::Clone for IVpnInterfaceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnInterfaceId {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d1011);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnInterfaceId_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAddressInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id_array_size: *mut u32, id: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnInterfaceIdFactory(::windows_core::IUnknown);
impl IVpnInterfaceIdFactory {
    pub fn CreateVpnInterfaceId(&self, address: &[u8]) -> ::windows_core::Result<VpnInterfaceId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnInterfaceId)(::windows_core::Interface::as_raw(this), address.len() as u32, address.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnInterfaceIdFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnInterfaceIdFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnInterfaceIdFactory {}
impl ::core::fmt::Debug for IVpnInterfaceIdFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnInterfaceIdFactory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnInterfaceIdFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{9e2ddca2-1712-4ce4-b179-8c652c6d1000}");
}
unsafe impl ::windows_core::Interface for IVpnInterfaceIdFactory {
    type Vtable = IVpnInterfaceIdFactory_Vtbl;
}
impl ::core::clone::Clone for IVpnInterfaceIdFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnInterfaceIdFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d1000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnInterfaceIdFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateVpnInterfaceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnManagementAgent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnManagementAgent {
    type Vtable = IVpnManagementAgent_Vtbl;
}
impl ::core::clone::Clone for IVpnManagementAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnManagementAgent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x193696cd_a5c4_4abe_852b_785be4cb3e34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnManagementAgent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AddProfileFromXmlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddProfileFromXmlAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AddProfileFromObjectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddProfileFromObjectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateProfileFromXmlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateProfileFromXmlAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateProfileFromObjectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateProfileFromObjectAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProfilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProfilesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectProfileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ConnectProfileWithPasswordCredentialAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, passwordcredential: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ConnectProfileWithPasswordCredentialAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectProfileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnNamespaceAssignment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnNamespaceAssignment {
    type Vtable = IVpnNamespaceAssignment_Vtbl;
}
impl ::core::clone::Clone for IVpnNamespaceAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnNamespaceAssignment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd7f7db18_307d_4c0e_bd62_8fa270bbadd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNamespaceAssignment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetNamespaceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetNamespaceList: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub NamespaceList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NamespaceList: usize,
    #[cfg(feature = "Foundation")]
    pub SetProxyAutoConfigUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProxyAutoConfigUri: usize,
    #[cfg(feature = "Foundation")]
    pub ProxyAutoConfigUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProxyAutoConfigUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnNamespaceInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnNamespaceInfo {
    type Vtable = IVpnNamespaceInfo_Vtbl;
}
impl ::core::clone::Clone for IVpnNamespaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnNamespaceInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30edfb43_444f_44c5_8167_a35a91f1af94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNamespaceInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetDnsServers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetDnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DnsServers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DnsServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetWebProxyServers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetWebProxyServers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub WebProxyServers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    WebProxyServers: usize,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnNamespaceInfoFactory(::windows_core::IUnknown);
impl IVpnNamespaceInfoFactory {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnNamespaceInfo<P0, P1>(&self, name: &::windows_core::HSTRING, dnsserverlist: P0, proxyserverlist: P1) -> ::windows_core::Result<VpnNamespaceInfo>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnNamespaceInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), dnsserverlist.try_into_param()?.abi(), proxyserverlist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnNamespaceInfoFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnNamespaceInfoFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnNamespaceInfoFactory {}
impl ::core::fmt::Debug for IVpnNamespaceInfoFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnNamespaceInfoFactory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnNamespaceInfoFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{cb3e951a-b0ce-442b-acbb-5f99b202c31c}");
}
unsafe impl ::windows_core::Interface for IVpnNamespaceInfoFactory {
    type Vtable = IVpnNamespaceInfoFactory_Vtbl;
}
impl ::core::clone::Clone for IVpnNamespaceInfoFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnNamespaceInfoFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb3e951a_b0ce_442b_acbb_5f99b202c31c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNamespaceInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateVpnNamespaceInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, dnsserverlist: *mut ::core::ffi::c_void, proxyserverlist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateVpnNamespaceInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnNativeProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnNativeProfile {
    type Vtable = IVpnNativeProfile_Vtbl;
}
impl ::core::clone::Clone for IVpnNativeProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnNativeProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4aee29e_6417_4333_9842_f0a66db69802);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNativeProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Servers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Servers: usize,
    pub RoutingPolicyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows_core::HRESULT,
    pub SetRoutingPolicyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows_core::HRESULT,
    pub NativeProtocolType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnNativeProtocolType) -> ::windows_core::HRESULT,
    pub SetNativeProtocolType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnNativeProtocolType) -> ::windows_core::HRESULT,
    pub UserAuthenticationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows_core::HRESULT,
    pub SetUserAuthenticationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows_core::HRESULT,
    pub TunnelAuthenticationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows_core::HRESULT,
    pub SetTunnelAuthenticationMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows_core::HRESULT,
    pub EapConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEapConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnNativeProfile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnNativeProfile2 {
    type Vtable = IVpnNativeProfile2_Vtbl;
}
impl ::core::clone::Clone for IVpnNativeProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnNativeProfile2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fec2467_cdb5_4ac7_b5a3_0afb5ec47682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnNativeProfile2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequireVpnClientAppUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequireVpnClientAppUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPacketBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPacketBuffer {
    type Vtable = IVpnPacketBuffer_Vtbl;
}
impl ::core::clone::Clone for IVpnPacketBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPacketBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2f891fc_4d5c_4a63_b70d_4e307eacce55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows_core::HRESULT,
    pub SetTransportAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub TransportAffinity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPacketBuffer2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPacketBuffer2 {
    type Vtable = IVpnPacketBuffer2_Vtbl;
}
impl ::core::clone::Clone for IVpnPacketBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPacketBuffer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x665e91f0_8805_4bf5_a619_2e84882e6b4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBuffer2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPacketBuffer3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPacketBuffer3 {
    type Vtable = IVpnPacketBuffer3_Vtbl;
}
impl ::core::clone::Clone for IVpnPacketBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPacketBuffer3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe256072f_107b_4c40_b127_5bc53e0ad960);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBuffer3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTransportContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TransportContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnPacketBufferFactory(::windows_core::IUnknown);
impl IVpnPacketBufferFactory {
    pub fn CreateVpnPacketBuffer<P0>(&self, parentbuffer: P0, offset: u32, length: u32) -> ::windows_core::Result<VpnPacketBuffer>
    where
        P0: ::windows_core::IntoParam<VpnPacketBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnPacketBuffer)(::windows_core::Interface::as_raw(this), parentbuffer.into_param().abi(), offset, length, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnPacketBufferFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnPacketBufferFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnPacketBufferFactory {}
impl ::core::fmt::Debug for IVpnPacketBufferFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnPacketBufferFactory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnPacketBufferFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{9e2ddca2-1712-4ce4-b179-8c652c6d9999}");
}
unsafe impl ::windows_core::Interface for IVpnPacketBufferFactory {
    type Vtable = IVpnPacketBufferFactory_Vtbl;
}
impl ::core::clone::Clone for IVpnPacketBufferFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPacketBufferFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e2ddca2_1712_4ce4_b179_8c652c6d9999);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBufferFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateVpnPacketBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentbuffer: *mut ::core::ffi::c_void, offset: u32, length: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPacketBufferList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPacketBufferList {
    type Vtable = IVpnPacketBufferList_Vtbl;
}
impl ::core::clone::Clone for IVpnPacketBufferList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPacketBufferList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2f891fc_4d5c_4a63_b70d_4e307eacce77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBufferList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddAtBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAtBegin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPacketBufferList2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPacketBufferList2 {
    type Vtable = IVpnPacketBufferList2_Vtbl;
}
impl ::core::clone::Clone for IVpnPacketBufferList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPacketBufferList2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e7acfe5_ea1e_482a_8d98_c065f57d89ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPacketBufferList2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddLeadingPacket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveLeadingPacket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddTrailingPacket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveTrailingPacket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPickedCredential(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPickedCredential {
    type Vtable = IVpnPickedCredential_Vtbl;
}
impl ::core::clone::Clone for IVpnPickedCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPickedCredential {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a793ac7_8854_4e52_ad97_24dd9a842bce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPickedCredential_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub PasskeyCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    PasskeyCredential: usize,
    pub AdditionalPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Credentials")]
    pub OldPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OldPasswordCredential: usize,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnPlugIn(::windows_core::IUnknown);
impl IVpnPlugIn {
    pub fn Connect<P0>(&self, channel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Connect)(::windows_core::Interface::as_raw(this), channel.into_param().abi()).ok() }
    }
    pub fn Disconnect<P0>(&self, channel: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Disconnect)(::windows_core::Interface::as_raw(this), channel.into_param().abi()).ok() }
    }
    pub fn GetKeepAlivePayload<P0>(&self, channel: P0, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnChannel>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetKeepAlivePayload)(::windows_core::Interface::as_raw(this), channel.into_param().abi(), keepalivepacket as *mut _ as _).ok() }
    }
    pub fn Encapsulate<P0, P1, P2>(&self, channel: P0, packets: P1, encapulatedpackets: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnChannel>,
        P1: ::windows_core::IntoParam<VpnPacketBufferList>,
        P2: ::windows_core::IntoParam<VpnPacketBufferList>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Encapsulate)(::windows_core::Interface::as_raw(this), channel.into_param().abi(), packets.into_param().abi(), encapulatedpackets.into_param().abi()).ok() }
    }
    pub fn Decapsulate<P0, P1, P2, P3>(&self, channel: P0, encapbuffer: P1, decapsulatedpackets: P2, controlpacketstosend: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnChannel>,
        P1: ::windows_core::IntoParam<VpnPacketBuffer>,
        P2: ::windows_core::IntoParam<VpnPacketBufferList>,
        P3: ::windows_core::IntoParam<VpnPacketBufferList>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Decapsulate)(::windows_core::Interface::as_raw(this), channel.into_param().abi(), encapbuffer.into_param().abi(), decapsulatedpackets.into_param().abi(), controlpacketstosend.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnPlugIn, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnPlugIn {}
impl ::core::fmt::Debug for IVpnPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnPlugIn").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnPlugIn {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ceb78d07-d0a8-4703-a091-c8c2c0915bc4}");
}
unsafe impl ::windows_core::Interface for IVpnPlugIn {
    type Vtable = IVpnPlugIn_Vtbl;
}
impl ::core::clone::Clone for IVpnPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPlugIn {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xceb78d07_d0a8_4703_a091_c8c2c0915bc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPlugIn_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetKeepAlivePayload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, keepalivepacket: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Encapsulate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, packets: *mut ::core::ffi::c_void, encapulatedpackets: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Decapsulate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channel: *mut ::core::ffi::c_void, encapbuffer: *mut ::core::ffi::c_void, decapsulatedpackets: *mut ::core::ffi::c_void, controlpacketstosend: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPlugInProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPlugInProfile {
    type Vtable = IVpnPlugInProfile_Vtbl;
}
impl ::core::clone::Clone for IVpnPlugInProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPlugInProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0edf0da4_4f00_4589_8d7b_4bf988f6542c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPlugInProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ServerUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ServerUris: usize,
    pub CustomConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCustomConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VpnPluginPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetVpnPluginPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnPlugInProfile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnPlugInProfile2 {
    type Vtable = IVpnPlugInProfile2_Vtbl;
}
impl ::core::clone::Clone for IVpnPlugInProfile2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnPlugInProfile2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x611c4892_cf94_4ad6_ba99_00f4ff34565e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnPlugInProfile2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequireVpnClientAppUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRequireVpnClientAppUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnProfile(::windows_core::IUnknown);
impl IVpnProfile {
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProfileName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProfileName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppTriggers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Routes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainNameInfoList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrafficFilters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RememberCredentials(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RememberCredentials)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRememberCredentials)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysOn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysOn)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnProfile {}
impl ::core::fmt::Debug for IVpnProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnProfile").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{7875b751-b0d7-43db-8a93-d3fe2479e56a}");
}
unsafe impl ::windows_core::Interface for IVpnProfile {
    type Vtable = IVpnProfile_Vtbl;
}
impl ::core::clone::Clone for IVpnProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7875b751_b0d7_43db_8a93_d3fe2479e56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppTriggers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppTriggers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Routes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Routes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DomainNameInfoList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DomainNameInfoList: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TrafficFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrafficFilters: usize,
    pub RememberCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRememberCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AlwaysOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAlwaysOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnRoute(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnRoute {
    type Vtable = IVpnRoute_Vtbl;
}
impl ::core::clone::Clone for IVpnRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnRoute {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5731b83_0969_4699_938e_7776db29cfb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnRoute_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPrefixSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub PrefixSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnRouteAssignment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnRouteAssignment {
    type Vtable = IVpnRouteAssignment_Vtbl;
}
impl ::core::clone::Clone for IVpnRouteAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnRouteAssignment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb64de22_ce39_4a76_9550_f61039f80e48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnRouteAssignment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv4InclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv4InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv6InclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv6InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv4InclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv4InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv6InclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv6InclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv4ExclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv4ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetIpv6ExclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetIpv6ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv4ExclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv4ExclusionRoutes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Ipv6ExclusionRoutes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Ipv6ExclusionRoutes: usize,
    pub SetExcludeLocalSubnets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ExcludeLocalSubnets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct IVpnRouteFactory(::windows_core::IUnknown);
impl IVpnRouteFactory {
    pub fn CreateVpnRoute<P0>(&self, address: P0, prefixsize: u8) -> ::windows_core::Result<VpnRoute>
    where
        P0: ::windows_core::IntoParam<super::HostName>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnRoute)(::windows_core::Interface::as_raw(this), address.into_param().abi(), prefixsize, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVpnRouteFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IVpnRouteFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVpnRouteFactory {}
impl ::core::fmt::Debug for IVpnRouteFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVpnRouteFactory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IVpnRouteFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{bdeab5ff-45cf-4b99-83fb-db3bc2672b02}");
}
unsafe impl ::windows_core::Interface for IVpnRouteFactory {
    type Vtable = IVpnRouteFactory_Vtbl;
}
impl ::core::clone::Clone for IVpnRouteFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnRouteFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbdeab5ff_45cf_4b99_83fb_db3bc2672b02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnRouteFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateVpnRoute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void, prefixsize: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnSystemHealth(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnSystemHealth {
    type Vtable = IVpnSystemHealth_Vtbl;
}
impl ::core::clone::Clone for IVpnSystemHealth {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnSystemHealth {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99a8f8af_c0ee_4e75_817a_f231aee5123d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnSystemHealth_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub StatementOfHealth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    StatementOfHealth: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnTrafficFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnTrafficFilter {
    type Vtable = IVpnTrafficFilter_Vtbl;
}
impl ::core::clone::Clone for IVpnTrafficFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnTrafficFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f691b60_6c9f_47f5_ac36_bb1b042e2c50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnTrafficFilter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AppClaims: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppClaims: usize,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnIPProtocol) -> ::windows_core::HRESULT,
    pub SetProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnIPProtocol) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalPortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalPortRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemotePortRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemotePortRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalAddressRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalAddressRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoteAddressRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoteAddressRanges: usize,
    pub RoutingPolicyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows_core::HRESULT,
    pub SetRoutingPolicyType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnTrafficFilterAssignment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnTrafficFilterAssignment {
    type Vtable = IVpnTrafficFilterAssignment_Vtbl;
}
impl ::core::clone::Clone for IVpnTrafficFilterAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnTrafficFilterAssignment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56ccd45c_e664_471e_89cd_601603b9e0f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnTrafficFilterAssignment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub TrafficFilterList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TrafficFilterList: usize,
    pub AllowOutbound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowOutbound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowInbound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowInbound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVpnTrafficFilterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVpnTrafficFilterFactory {
    type Vtable = IVpnTrafficFilterFactory_Vtbl;
}
impl ::core::clone::Clone for IVpnTrafficFilterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVpnTrafficFilterFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x480d41d5_7f99_474c_86ee_96df168318f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVpnTrafficFilterFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnAppId(::windows_core::IUnknown);
impl VpnAppId {
    pub fn Type(&self) -> ::windows_core::Result<VpnAppIdType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: VpnAppIdType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(r#type: VpnAppIdType, value: &::windows_core::HSTRING) -> ::windows_core::Result<VpnAppId> {
        Self::IVpnAppIdFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), r#type, ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVpnAppIdFactory<R, F: FnOnce(&IVpnAppIdFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnAppId, IVpnAppIdFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnAppId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnAppId {}
impl ::core::fmt::Debug for VpnAppId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnAppId").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnAppId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnAppId;{7b06a635-5c58-41d9-94a7-bfbcf1d8ca54})");
}
impl ::core::clone::Clone for VpnAppId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnAppId {
    type Vtable = IVpnAppId_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnAppId {
    const IID: ::windows_core::GUID = <IVpnAppId as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnAppId {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnAppId";
}
::windows_core::imp::interface_hierarchy!(VpnAppId, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnAppId {}
unsafe impl ::core::marker::Sync for VpnAppId {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnChannel(::windows_core::IUnknown);
impl VpnChannel {
    pub fn AssociateTransport<P0, P1>(&self, mainoutertunneltransport: P0, optionaloutertunneltransport: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AssociateTransport)(::windows_core::Interface::as_raw(this), mainoutertunneltransport.into_param().abi(), optionaloutertunneltransport.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Start<P0, P1, P2, P3, P4, P5, P6>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, routescope: P3, namespacescope: P4, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: P5, optionaloutertunneltransport: P6) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: ::windows_core::IntoParam<VpnInterfaceId>,
        P3: ::windows_core::IntoParam<VpnRouteAssignment>,
        P4: ::windows_core::IntoParam<VpnNamespaceAssignment>,
        P5: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P6: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this), assignedclientipv4list.try_into_param()?.abi(), assignedclientipv6list.try_into_param()?.abi(), vpninterfaceid.into_param().abi(), routescope.into_param().abi(), namespacescope.into_param().abi(), mtusize, maxframesize, optimizeforlowcostnetwork, mainoutertunneltransport.into_param().abi(), optionaloutertunneltransport.into_param().abi()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn RequestCredentials<P0>(&self, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: P0) -> ::windows_core::Result<VpnPickedCredential>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCredentials)(::windows_core::Interface::as_raw(this), credtype, isretry, issinglesignoncredential, certificate.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn RequestVpnPacketBuffer(&self, r#type: VpnDataPathType, vpnpacketbuffer: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestVpnPacketBuffer)(::windows_core::Interface::as_raw(this), r#type, vpnpacketbuffer as *mut _ as _).ok() }
    }
    pub fn LogDiagnosticMessage(&self, message: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LogDiagnosticMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(message)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<VpnChannelConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActivityChange<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivityChange)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivityChange(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActivityChange)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetPlugInContext<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlugInContext)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PlugInContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlugInContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SystemHealth(&self) -> ::windows_core::Result<VpnSystemHealth> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemHealth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestCustomPrompt<P0>(&self, customprompt: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RequestCustomPrompt)(::windows_core::Interface::as_raw(this), customprompt.try_into_param()?.abi()).ok() }
    }
    pub fn SetErrorMessage(&self, message: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetErrorMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(message)).ok() }
    }
    pub fn SetAllowedSslTlsVersions<P0>(&self, tunneltransport: P0, usetls12: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowedSslTlsVersions)(::windows_core::Interface::as_raw(this), tunneltransport.into_param().abi(), usetls12).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithMainTransport<P0, P1, P2, P3, P4, P5>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, assignedroutes: P3, assigneddomainname: P4, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: P5) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: ::windows_core::IntoParam<VpnInterfaceId>,
        P3: ::windows_core::IntoParam<VpnRouteAssignment>,
        P4: ::windows_core::IntoParam<VpnDomainNameAssignment>,
        P5: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartWithMainTransport)(::windows_core::Interface::as_raw(this), assignedclientipv4list.try_into_param()?.abi(), assignedclientipv6list.try_into_param()?.abi(), vpninterfaceid.into_param().abi(), assignedroutes.into_param().abi(), assigneddomainname.into_param().abi(), mtusize, maxframesize, reserved, mainoutertunneltransport.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartExistingTransports<P0, P1, P2, P3, P4>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, assignedroutes: P3, assigneddomainname: P4, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: ::windows_core::IntoParam<VpnInterfaceId>,
        P3: ::windows_core::IntoParam<VpnRouteAssignment>,
        P4: ::windows_core::IntoParam<VpnDomainNameAssignment>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartExistingTransports)(::windows_core::Interface::as_raw(this), assignedclientipv4list.try_into_param()?.abi(), assignedclientipv6list.try_into_param()?.abi(), vpninterfaceid.into_param().abi(), assignedroutes.into_param().abi(), assigneddomainname.into_param().abi(), mtusize, maxframesize, reserved).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActivityStateChange<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivityStateChange)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivityStateChange(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActivityStateChange)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetVpnSendPacketBuffer(&self) -> ::windows_core::Result<VpnPacketBuffer> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVpnSendPacketBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetVpnReceivePacketBuffer(&self) -> ::windows_core::Result<VpnPacketBuffer> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVpnReceivePacketBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestCustomPromptAsync<P0>(&self, custompromptelement: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCustomPromptAsync)(::windows_core::Interface::as_raw(this), custompromptelement.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates"))]
    pub fn RequestCredentialsWithCertificateAsync<P0>(&self, credtype: VpnCredentialType, credoptions: u32, certificate: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Cryptography::Certificates::Certificate>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCredentialsWithCertificateAsync)(::windows_core::Interface::as_raw(this), credtype, credoptions, certificate.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCredentialsWithOptionsAsync(&self, credtype: VpnCredentialType, credoptions: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCredentialsWithOptionsAsync)(::windows_core::Interface::as_raw(this), credtype, credoptions, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCredentialsSimpleAsync(&self, credtype: VpnCredentialType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCredentialsSimpleAsync)(::windows_core::Interface::as_raw(this), credtype, &mut result__).from_abi(result__)
        }
    }
    pub fn TerminateConnection(&self, message: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).TerminateConnection)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(message)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithTrafficFilter<P0, P1, P2, P3, P4, P5, P6, P7>(&self, assignedclientipv4list: P0, assignedclientipv6list: P1, vpninterfaceid: P2, assignedroutes: P3, assignednamespace: P4, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: P5, optionaloutertunneltransport: P6, assignedtrafficfilters: P7) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<super::HostName>>,
        P2: ::windows_core::IntoParam<VpnInterfaceId>,
        P3: ::windows_core::IntoParam<VpnRouteAssignment>,
        P4: ::windows_core::IntoParam<VpnDomainNameAssignment>,
        P5: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P6: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P7: ::windows_core::IntoParam<VpnTrafficFilterAssignment>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartWithTrafficFilter)(::windows_core::Interface::as_raw(this), assignedclientipv4list.try_into_param()?.abi(), assignedclientipv6list.try_into_param()?.abi(), vpninterfaceid.into_param().abi(), assignedroutes.into_param().abi(), assignednamespace.into_param().abi(), mtusize, maxframesize, reserved, mainoutertunneltransport.into_param().abi(), optionaloutertunneltransport.into_param().abi(), assignedtrafficfilters.into_param().abi()).ok() }
    }
    pub fn AddAndAssociateTransport<P0, P1>(&self, transport: P0, context: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddAndAssociateTransport)(::windows_core::Interface::as_raw(this), transport.into_param().abi(), context.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithMultipleTransports<P0, P1, P2, P3, P4, P5, P6>(&self, assignedclientipv4addresses: P0, assignedclientipv6addresses: P1, vpninterfaceid: P2, assignedroutes: P3, assignednamespace: P4, mtusize: u32, maxframesize: u32, reserved: bool, transports: P5, assignedtrafficfilters: P6) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P2: ::windows_core::IntoParam<VpnInterfaceId>,
        P3: ::windows_core::IntoParam<VpnRouteAssignment>,
        P4: ::windows_core::IntoParam<VpnDomainNameAssignment>,
        P5: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::IInspectable>>,
        P6: ::windows_core::IntoParam<VpnTrafficFilterAssignment>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartWithMultipleTransports)(::windows_core::Interface::as_raw(this), assignedclientipv4addresses.try_into_param()?.abi(), assignedclientipv6addresses.try_into_param()?.abi(), vpninterfaceid.into_param().abi(), assignedroutes.into_param().abi(), assignednamespace.into_param().abi(), mtusize, maxframesize, reserved, transports.try_into_param()?.abi(), assignedtrafficfilters.into_param().abi()).ok() }
    }
    pub fn ReplaceAndAssociateTransport<P0, P1>(&self, transport: P0, context: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAndAssociateTransport)(::windows_core::Interface::as_raw(this), transport.into_param().abi(), context.into_param().abi()).ok() }
    }
    pub fn StartReconnectingTransport<P0, P1>(&self, transport: P0, context: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).StartReconnectingTransport)(::windows_core::Interface::as_raw(this), transport.into_param().abi(), context.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn GetSlotTypeForTransportContext<P0>(&self, context: P0) -> ::windows_core::Result<super::Sockets::ControlChannelTriggerStatus>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSlotTypeForTransportContext)(::windows_core::Interface::as_raw(this), context.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentRequestTransportContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentRequestTransportContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppendVpnReceivePacketBuffer<P0>(&self, decapsulatedpacketbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnPacketBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendVpnReceivePacketBuffer)(::windows_core::Interface::as_raw(this), decapsulatedpacketbuffer.into_param().abi()).ok() }
    }
    pub fn AppendVpnSendPacketBuffer<P0>(&self, encapsulatedpacketbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnPacketBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AppendVpnSendPacketBuffer)(::windows_core::Interface::as_raw(this), encapsulatedpacketbuffer.into_param().abi()).ok() }
    }
    pub fn FlushVpnReceivePacketBuffers(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FlushVpnReceivePacketBuffers)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FlushVpnSendPacketBuffers(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).FlushVpnSendPacketBuffers)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivateForeground<P0>(&self, packagerelativeappid: &::windows_core::HSTRING, sharedcontext: P0) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnChannel6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivateForeground)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagerelativeappid), sharedcontext.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ProcessEventAsync<P0, P1>(thirdpartyplugin: P0, event: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
        P1: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IVpnChannelStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ProcessEventAsync)(::windows_core::Interface::as_raw(this), thirdpartyplugin.into_param().abi(), event.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IVpnChannelStatics<R, F: FnOnce(&IVpnChannelStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnChannel, IVpnChannelStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnChannel {}
impl ::core::fmt::Debug for VpnChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnChannel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnChannel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannel;{4ac78d07-d1a8-4303-a091-c8d2e0915bc3})");
}
impl ::core::clone::Clone for VpnChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnChannel {
    type Vtable = IVpnChannel_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnChannel {
    const IID: ::windows_core::GUID = <IVpnChannel as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnChannel {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannel";
}
::windows_core::imp::interface_hierarchy!(VpnChannel, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnChannel {}
unsafe impl ::core::marker::Sync for VpnChannel {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnChannelActivityEventArgs(::windows_core::IUnknown);
impl VpnChannelActivityEventArgs {
    pub fn Type(&self) -> ::windows_core::Result<VpnChannelActivityEventType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnChannelActivityEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnChannelActivityEventArgs {}
impl ::core::fmt::Debug for VpnChannelActivityEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnChannelActivityEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnChannelActivityEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannelActivityEventArgs;{a36c88f2-afdc-4775-855d-d4ac0a35fc55})");
}
impl ::core::clone::Clone for VpnChannelActivityEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnChannelActivityEventArgs {
    type Vtable = IVpnChannelActivityEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnChannelActivityEventArgs {
    const IID: ::windows_core::GUID = <IVpnChannelActivityEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnChannelActivityEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelActivityEventArgs";
}
::windows_core::imp::interface_hierarchy!(VpnChannelActivityEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnChannelActivityEventArgs {}
unsafe impl ::core::marker::Sync for VpnChannelActivityEventArgs {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnChannelActivityStateChangedArgs(::windows_core::IUnknown);
impl VpnChannelActivityStateChangedArgs {
    pub fn ActivityState(&self) -> ::windows_core::Result<VpnChannelActivityEventType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivityState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnChannelActivityStateChangedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnChannelActivityStateChangedArgs {}
impl ::core::fmt::Debug for VpnChannelActivityStateChangedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnChannelActivityStateChangedArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnChannelActivityStateChangedArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs;{3d750565-fdc0-4bbe-a23b-45fffc6d97a1})");
}
impl ::core::clone::Clone for VpnChannelActivityStateChangedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnChannelActivityStateChangedArgs {
    type Vtable = IVpnChannelActivityStateChangedArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnChannelActivityStateChangedArgs {
    const IID: ::windows_core::GUID = <IVpnChannelActivityStateChangedArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnChannelActivityStateChangedArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelActivityStateChangedArgs";
}
::windows_core::imp::interface_hierarchy!(VpnChannelActivityStateChangedArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnChannelActivityStateChangedArgs {}
unsafe impl ::core::marker::Sync for VpnChannelActivityStateChangedArgs {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnChannelConfiguration(::windows_core::IUnknown);
impl VpnChannelConfiguration {
    pub fn ServerServiceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerServiceName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerHostNameList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerHostNameList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CustomField(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomField)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerUris(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = &::windows_core::ComInterface::cast::<IVpnChannelConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerUris)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnChannelConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnChannelConfiguration {}
impl ::core::fmt::Debug for VpnChannelConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnChannelConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnChannelConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnChannelConfiguration;{0e2ddca2-2012-4fe4-b179-8c652c6d107e})");
}
impl ::core::clone::Clone for VpnChannelConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnChannelConfiguration {
    type Vtable = IVpnChannelConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnChannelConfiguration {
    const IID: ::windows_core::GUID = <IVpnChannelConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnChannelConfiguration {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnChannelConfiguration";
}
::windows_core::imp::interface_hierarchy!(VpnChannelConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnChannelConfiguration {}
unsafe impl ::core::marker::Sync for VpnChannelConfiguration {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCredential(::windows_core::IUnknown);
impl VpnCredential {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PasskeyCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(feature = "Security_Cryptography_Certificates")]
    pub fn CertificateCredential(&self) -> ::windows_core::Result<super::super::Security::Cryptography::Certificates::Certificate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CertificateCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AdditionalPin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalPin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldPasswordCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCredential {}
impl ::core::fmt::Debug for VpnCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCredential").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCredential {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCredential;{b7e78af3-a46d-404b-8729-1832522853ac})");
}
impl ::core::clone::Clone for VpnCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCredential {
    type Vtable = IVpnCredential_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCredential {
    const IID: ::windows_core::GUID = <IVpnCredential as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCredential";
}
::windows_core::imp::interface_hierarchy!(VpnCredential, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCredential> for VpnCredential {}
unsafe impl ::core::marker::Send for VpnCredential {}
unsafe impl ::core::marker::Sync for VpnCredential {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomCheckBox(::windows_core::IUnknown);
impl VpnCustomCheckBox {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomCheckBox, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetInitialCheckState(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitialCheckState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InitialCheckState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitialCheckState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Checked(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Checked)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBordered)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bordered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomCheckBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomCheckBox {}
impl ::core::fmt::Debug for VpnCustomCheckBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomCheckBox").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomCheckBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomCheckBox;{43878753-03c5-4e61-93d7-a957714c4282})");
}
impl ::core::clone::Clone for VpnCustomCheckBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomCheckBox {
    type Vtable = IVpnCustomCheckBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomCheckBox {
    const IID: ::windows_core::GUID = <IVpnCustomCheckBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomCheckBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomCheckBox";
}
::windows_core::imp::interface_hierarchy!(VpnCustomCheckBox, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPrompt> for VpnCustomCheckBox {}
unsafe impl ::core::marker::Send for VpnCustomCheckBox {}
unsafe impl ::core::marker::Sync for VpnCustomCheckBox {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomComboBox(::windows_core::IUnknown);
impl VpnCustomComboBox {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomComboBox, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetOptionsText<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOptionsText)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OptionsText(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OptionsText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Selected(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Selected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBordered)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bordered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomComboBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomComboBox {}
impl ::core::fmt::Debug for VpnCustomComboBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomComboBox").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomComboBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomComboBox;{9a24158e-dba1-4c6f-8270-dcf3c9761c4c})");
}
impl ::core::clone::Clone for VpnCustomComboBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomComboBox {
    type Vtable = IVpnCustomComboBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomComboBox {
    const IID: ::windows_core::GUID = <IVpnCustomComboBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomComboBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomComboBox";
}
::windows_core::imp::interface_hierarchy!(VpnCustomComboBox, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPrompt> for VpnCustomComboBox {}
unsafe impl ::core::marker::Send for VpnCustomComboBox {}
unsafe impl ::core::marker::Sync for VpnCustomComboBox {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomEditBox(::windows_core::IUnknown);
impl VpnCustomEditBox {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomEditBox, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDefaultText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DefaultText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNoEcho(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNoEcho)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NoEcho(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NoEcho)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBordered)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bordered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomEditBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomEditBox {}
impl ::core::fmt::Debug for VpnCustomEditBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomEditBox").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomEditBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomEditBox;{3002d9a0-cfbf-4c0b-8f3c-66f503c20b39})");
}
impl ::core::clone::Clone for VpnCustomEditBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomEditBox {
    type Vtable = IVpnCustomEditBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomEditBox {
    const IID: ::windows_core::GUID = <IVpnCustomEditBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomEditBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomEditBox";
}
::windows_core::imp::interface_hierarchy!(VpnCustomEditBox, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPrompt> for VpnCustomEditBox {}
unsafe impl ::core::marker::Send for VpnCustomEditBox {}
unsafe impl ::core::marker::Sync for VpnCustomEditBox {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomErrorBox(::windows_core::IUnknown);
impl VpnCustomErrorBox {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomErrorBox, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBordered)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bordered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomErrorBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomErrorBox {}
impl ::core::fmt::Debug for VpnCustomErrorBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomErrorBox").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomErrorBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomErrorBox;{9ec4efb2-c942-42af-b223-588b48328721})");
}
impl ::core::clone::Clone for VpnCustomErrorBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomErrorBox {
    type Vtable = IVpnCustomErrorBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomErrorBox {
    const IID: ::windows_core::GUID = <IVpnCustomErrorBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomErrorBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomErrorBox";
}
::windows_core::imp::interface_hierarchy!(VpnCustomErrorBox, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPrompt> for VpnCustomErrorBox {}
unsafe impl ::core::marker::Send for VpnCustomErrorBox {}
unsafe impl ::core::marker::Sync for VpnCustomErrorBox {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomPromptBooleanInput(::windows_core::IUnknown);
impl VpnCustomPromptBooleanInput {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomPromptBooleanInput, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetInitialValue(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitialValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InitialValue(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitialValue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmphasized)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emphasized)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomPromptBooleanInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomPromptBooleanInput {}
impl ::core::fmt::Debug for VpnCustomPromptBooleanInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomPromptBooleanInput").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomPromptBooleanInput {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptBooleanInput;{c4c9a69e-ff47-4527-9f27-a49292019979})");
}
impl ::core::clone::Clone for VpnCustomPromptBooleanInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomPromptBooleanInput {
    type Vtable = IVpnCustomPromptBooleanInput_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomPromptBooleanInput {
    const IID: ::windows_core::GUID = <IVpnCustomPromptBooleanInput as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomPromptBooleanInput {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptBooleanInput";
}
::windows_core::imp::interface_hierarchy!(VpnCustomPromptBooleanInput, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPromptElement> for VpnCustomPromptBooleanInput {}
unsafe impl ::core::marker::Send for VpnCustomPromptBooleanInput {}
unsafe impl ::core::marker::Sync for VpnCustomPromptBooleanInput {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomPromptOptionSelector(::windows_core::IUnknown);
impl VpnCustomPromptOptionSelector {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomPromptOptionSelector, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmphasized)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emphasized)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Options)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectedIndex(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomPromptOptionSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomPromptOptionSelector {}
impl ::core::fmt::Debug for VpnCustomPromptOptionSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomPromptOptionSelector").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomPromptOptionSelector {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptOptionSelector;{3b8f34d9-8ec1-4e95-9a4e-7ba64d38f330})");
}
impl ::core::clone::Clone for VpnCustomPromptOptionSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomPromptOptionSelector {
    type Vtable = IVpnCustomPromptOptionSelector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomPromptOptionSelector {
    const IID: ::windows_core::GUID = <IVpnCustomPromptOptionSelector as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomPromptOptionSelector {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptOptionSelector";
}
::windows_core::imp::interface_hierarchy!(VpnCustomPromptOptionSelector, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPromptElement> for VpnCustomPromptOptionSelector {}
unsafe impl ::core::marker::Send for VpnCustomPromptOptionSelector {}
unsafe impl ::core::marker::Sync for VpnCustomPromptOptionSelector {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomPromptText(::windows_core::IUnknown);
impl VpnCustomPromptText {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomPromptText, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmphasized)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emphasized)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomPromptText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomPromptText {}
impl ::core::fmt::Debug for VpnCustomPromptText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomPromptText").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomPromptText {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptText;{3bc8bdee-3a42-49a3-abdd-07b2edea752d})");
}
impl ::core::clone::Clone for VpnCustomPromptText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomPromptText {
    type Vtable = IVpnCustomPromptText_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomPromptText {
    const IID: ::windows_core::GUID = <IVpnCustomPromptText as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomPromptText {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptText";
}
::windows_core::imp::interface_hierarchy!(VpnCustomPromptText, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPromptElement> for VpnCustomPromptText {}
unsafe impl ::core::marker::Send for VpnCustomPromptText {}
unsafe impl ::core::marker::Sync for VpnCustomPromptText {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomPromptTextInput(::windows_core::IUnknown);
impl VpnCustomPromptTextInput {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomPromptTextInput, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEmphasized(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEmphasized)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Emphasized(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPromptElement>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Emphasized)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPlaceholderText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaceholderText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PlaceholderText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaceholderText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsTextHidden(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsTextHidden)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTextHidden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextHidden)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomPromptTextInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomPromptTextInput {}
impl ::core::fmt::Debug for VpnCustomPromptTextInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomPromptTextInput").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomPromptTextInput {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomPromptTextInput;{c9da9c75-913c-47d5-88ba-48fc48930235})");
}
impl ::core::clone::Clone for VpnCustomPromptTextInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomPromptTextInput {
    type Vtable = IVpnCustomPromptTextInput_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomPromptTextInput {
    const IID: ::windows_core::GUID = <IVpnCustomPromptTextInput as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomPromptTextInput {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomPromptTextInput";
}
::windows_core::imp::interface_hierarchy!(VpnCustomPromptTextInput, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPromptElement> for VpnCustomPromptTextInput {}
unsafe impl ::core::marker::Send for VpnCustomPromptTextInput {}
unsafe impl ::core::marker::Sync for VpnCustomPromptTextInput {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnCustomTextBox(::windows_core::IUnknown);
impl VpnCustomTextBox {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnCustomTextBox, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompulsory(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompulsory)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Compulsory(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Compulsory)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBordered(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetBordered)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bordered(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnCustomPrompt>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bordered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnCustomTextBox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnCustomTextBox {}
impl ::core::fmt::Debug for VpnCustomTextBox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCustomTextBox").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCustomTextBox {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnCustomTextBox;{daa4c3ca-8f23-4d36-91f1-76d937827942})");
}
impl ::core::clone::Clone for VpnCustomTextBox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnCustomTextBox {
    type Vtable = IVpnCustomTextBox_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnCustomTextBox {
    const IID: ::windows_core::GUID = <IVpnCustomTextBox as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnCustomTextBox {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnCustomTextBox";
}
::windows_core::imp::interface_hierarchy!(VpnCustomTextBox, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnCustomPrompt> for VpnCustomTextBox {}
unsafe impl ::core::marker::Send for VpnCustomTextBox {}
unsafe impl ::core::marker::Sync for VpnCustomTextBox {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnDomainNameAssignment(::windows_core::IUnknown);
impl VpnDomainNameAssignment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnDomainNameAssignment, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainNameList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProxyAutoConfigurationUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyAutoConfigurationUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProxyAutoConfigurationUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProxyAutoConfigurationUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnDomainNameAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnDomainNameAssignment {}
impl ::core::fmt::Debug for VpnDomainNameAssignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnDomainNameAssignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnDomainNameAssignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnDomainNameAssignment;{4135b141-ccdb-49b5-9401-039a8ae767e9})");
}
impl ::core::clone::Clone for VpnDomainNameAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnDomainNameAssignment {
    type Vtable = IVpnDomainNameAssignment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnDomainNameAssignment {
    const IID: ::windows_core::GUID = <IVpnDomainNameAssignment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnDomainNameAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnDomainNameAssignment";
}
::windows_core::imp::interface_hierarchy!(VpnDomainNameAssignment, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnDomainNameAssignment {}
unsafe impl ::core::marker::Sync for VpnDomainNameAssignment {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnDomainNameInfo(::windows_core::IUnknown);
impl VpnDomainNameInfo {
    pub fn SetDomainName<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::HostName>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DomainName(&self) -> ::windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDomainNameType(&self, value: VpnDomainNameType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDomainNameType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DomainNameType(&self) -> ::windows_core::Result<VpnDomainNameType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainNameType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsServers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DnsServers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyServers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebProxyServers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyUris(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = &::windows_core::ComInterface::cast::<IVpnDomainNameInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebProxyUris)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnDomainNameInfo<P0, P1>(name: &::windows_core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: P0, proxyserverlist: P1) -> ::windows_core::Result<VpnDomainNameInfo>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::HostName>>,
    {
        Self::IVpnDomainNameInfoFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnDomainNameInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), nametype, dnsserverlist.try_into_param()?.abi(), proxyserverlist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVpnDomainNameInfoFactory<R, F: FnOnce(&IVpnDomainNameInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnDomainNameInfo, IVpnDomainNameInfoFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnDomainNameInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnDomainNameInfo {}
impl ::core::fmt::Debug for VpnDomainNameInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnDomainNameInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnDomainNameInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnDomainNameInfo;{ad2eb82f-ea8e-4f7a-843e-1a87e32e1b9a})");
}
impl ::core::clone::Clone for VpnDomainNameInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnDomainNameInfo {
    type Vtable = IVpnDomainNameInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnDomainNameInfo {
    const IID: ::windows_core::GUID = <IVpnDomainNameInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnDomainNameInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnDomainNameInfo";
}
::windows_core::imp::interface_hierarchy!(VpnDomainNameInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnDomainNameInfo {}
unsafe impl ::core::marker::Sync for VpnDomainNameInfo {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnForegroundActivatedEventArgs(::windows_core::IUnknown);
impl VpnForegroundActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SharedContext(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SharedContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActivationOperation(&self) -> ::windows_core::Result<VpnForegroundActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivationOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnForegroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnForegroundActivatedEventArgs {}
impl ::core::fmt::Debug for VpnForegroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnForegroundActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnForegroundActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnForegroundActivatedEventArgs;{85b465b0-cadb-4d70-ac92-543a24dc9ebc})");
}
impl ::core::clone::Clone for VpnForegroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnForegroundActivatedEventArgs {
    type Vtable = IVpnForegroundActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnForegroundActivatedEventArgs {
    const IID: ::windows_core::GUID = <IVpnForegroundActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnForegroundActivatedEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnForegroundActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VpnForegroundActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgs> for VpnForegroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for VpnForegroundActivatedEventArgs {}
unsafe impl ::core::marker::Send for VpnForegroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VpnForegroundActivatedEventArgs {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnForegroundActivationOperation(::windows_core::IUnknown);
impl VpnForegroundActivationOperation {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Complete<P0>(&self, result: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this), result.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for VpnForegroundActivationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnForegroundActivationOperation {}
impl ::core::fmt::Debug for VpnForegroundActivationOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnForegroundActivationOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnForegroundActivationOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnForegroundActivationOperation;{9e010d57-f17a-4bd5-9b6d-f984f1297d3c})");
}
impl ::core::clone::Clone for VpnForegroundActivationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnForegroundActivationOperation {
    type Vtable = IVpnForegroundActivationOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnForegroundActivationOperation {
    const IID: ::windows_core::GUID = <IVpnForegroundActivationOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnForegroundActivationOperation {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnForegroundActivationOperation";
}
::windows_core::imp::interface_hierarchy!(VpnForegroundActivationOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnForegroundActivationOperation {}
unsafe impl ::core::marker::Sync for VpnForegroundActivationOperation {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnInterfaceId(::windows_core::IUnknown);
impl VpnInterfaceId {
    pub fn GetAddressInfo(&self, id: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetAddressInfo)(::windows_core::Interface::as_raw(this), id.set_abi_len(), id as *mut _ as _).ok() }
    }
    pub fn CreateVpnInterfaceId(address: &[u8]) -> ::windows_core::Result<VpnInterfaceId> {
        Self::IVpnInterfaceIdFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnInterfaceId)(::windows_core::Interface::as_raw(this), address.len() as u32, address.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVpnInterfaceIdFactory<R, F: FnOnce(&IVpnInterfaceIdFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnInterfaceId, IVpnInterfaceIdFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnInterfaceId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnInterfaceId {}
impl ::core::fmt::Debug for VpnInterfaceId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnInterfaceId").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnInterfaceId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnInterfaceId;{9e2ddca2-1712-4ce4-b179-8c652c6d1011})");
}
impl ::core::clone::Clone for VpnInterfaceId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnInterfaceId {
    type Vtable = IVpnInterfaceId_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnInterfaceId {
    const IID: ::windows_core::GUID = <IVpnInterfaceId as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnInterfaceId {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnInterfaceId";
}
::windows_core::imp::interface_hierarchy!(VpnInterfaceId, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnInterfaceId {}
unsafe impl ::core::marker::Sync for VpnInterfaceId {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnManagementAgent(::windows_core::IUnknown);
impl VpnManagementAgent {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnManagementAgent, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddProfileFromXmlAsync(&self, xml: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddProfileFromXmlAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xml), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddProfileFromObjectAsync<P0>(&self, profile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: ::windows_core::TryIntoParam<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddProfileFromObjectAsync)(::windows_core::Interface::as_raw(this), profile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateProfileFromXmlAsync(&self, xml: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateProfileFromXmlAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(xml), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateProfileFromObjectAsync<P0>(&self, profile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: ::windows_core::TryIntoParam<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateProfileFromObjectAsync)(::windows_core::Interface::as_raw(this), profile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProfilesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IVpnProfile>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProfilesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteProfileAsync<P0>(&self, profile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: ::windows_core::TryIntoParam<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteProfileAsync)(::windows_core::Interface::as_raw(this), profile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectProfileAsync<P0>(&self, profile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: ::windows_core::TryIntoParam<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectProfileAsync)(::windows_core::Interface::as_raw(this), profile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ConnectProfileWithPasswordCredentialAsync<P0, P1>(&self, profile: P0, passwordcredential: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: ::windows_core::TryIntoParam<IVpnProfile>,
        P1: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectProfileWithPasswordCredentialAsync)(::windows_core::Interface::as_raw(this), profile.try_into_param()?.abi(), passwordcredential.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectProfileAsync<P0>(&self, profile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>
    where
        P0: ::windows_core::TryIntoParam<IVpnProfile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisconnectProfileAsync)(::windows_core::Interface::as_raw(this), profile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnManagementAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnManagementAgent {}
impl ::core::fmt::Debug for VpnManagementAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnManagementAgent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnManagementAgent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnManagementAgent;{193696cd-a5c4-4abe-852b-785be4cb3e34})");
}
impl ::core::clone::Clone for VpnManagementAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnManagementAgent {
    type Vtable = IVpnManagementAgent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnManagementAgent {
    const IID: ::windows_core::GUID = <IVpnManagementAgent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnManagementAgent {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnManagementAgent";
}
::windows_core::imp::interface_hierarchy!(VpnManagementAgent, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnManagementAgent {}
unsafe impl ::core::marker::Sync for VpnManagementAgent {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnNamespaceAssignment(::windows_core::IUnknown);
impl VpnNamespaceAssignment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnNamespaceAssignment, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetNamespaceList<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNamespaceList)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NamespaceList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NamespaceList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProxyAutoConfigUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProxyAutoConfigUri)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProxyAutoConfigUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProxyAutoConfigUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnNamespaceAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnNamespaceAssignment {}
impl ::core::fmt::Debug for VpnNamespaceAssignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnNamespaceAssignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnNamespaceAssignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnNamespaceAssignment;{d7f7db18-307d-4c0e-bd62-8fa270bbadd6})");
}
impl ::core::clone::Clone for VpnNamespaceAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnNamespaceAssignment {
    type Vtable = IVpnNamespaceAssignment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnNamespaceAssignment {
    const IID: ::windows_core::GUID = <IVpnNamespaceAssignment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnNamespaceAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNamespaceAssignment";
}
::windows_core::imp::interface_hierarchy!(VpnNamespaceAssignment, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnNamespaceAssignment {}
unsafe impl ::core::marker::Sync for VpnNamespaceAssignment {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnNamespaceInfo(::windows_core::IUnknown);
impl VpnNamespaceInfo {
    pub fn SetNamespace(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNamespace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Namespace(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Namespace)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetDnsServers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDnsServers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DnsServers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DnsServers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetWebProxyServers<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWebProxyServers)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn WebProxyServers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebProxyServers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateVpnNamespaceInfo<P0, P1>(name: &::windows_core::HSTRING, dnsserverlist: P0, proxyserverlist: P1) -> ::windows_core::Result<VpnNamespaceInfo>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<super::HostName>>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<super::HostName>>,
    {
        Self::IVpnNamespaceInfoFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnNamespaceInfo)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), dnsserverlist.try_into_param()?.abi(), proxyserverlist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVpnNamespaceInfoFactory<R, F: FnOnce(&IVpnNamespaceInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnNamespaceInfo, IVpnNamespaceInfoFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnNamespaceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnNamespaceInfo {}
impl ::core::fmt::Debug for VpnNamespaceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnNamespaceInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnNamespaceInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnNamespaceInfo;{30edfb43-444f-44c5-8167-a35a91f1af94})");
}
impl ::core::clone::Clone for VpnNamespaceInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnNamespaceInfo {
    type Vtable = IVpnNamespaceInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnNamespaceInfo {
    const IID: ::windows_core::GUID = <IVpnNamespaceInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnNamespaceInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNamespaceInfo";
}
::windows_core::imp::interface_hierarchy!(VpnNamespaceInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnNamespaceInfo {}
unsafe impl ::core::marker::Sync for VpnNamespaceInfo {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnNativeProfile(::windows_core::IUnknown);
impl VpnNativeProfile {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnNativeProfile, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Servers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Servers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoutingPolicyType(&self) -> ::windows_core::Result<VpnRoutingPolicyType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoutingPolicyType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoutingPolicyType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn NativeProtocolType(&self) -> ::windows_core::Result<VpnNativeProtocolType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NativeProtocolType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNativeProtocolType(&self, value: VpnNativeProtocolType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNativeProtocolType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn UserAuthenticationMethod(&self) -> ::windows_core::Result<VpnAuthenticationMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserAuthenticationMethod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUserAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserAuthenticationMethod)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TunnelAuthenticationMethod(&self) -> ::windows_core::Result<VpnAuthenticationMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TunnelAuthenticationMethod)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTunnelAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTunnelAuthenticationMethod)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EapConfiguration(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EapConfiguration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEapConfiguration(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEapConfiguration)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RequireVpnClientAppUI(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnNativeProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequireVpnClientAppUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnNativeProfile2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequireVpnClientAppUI)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ConnectionStatus(&self) -> ::windows_core::Result<VpnManagementConnectionStatus> {
        let this = &::windows_core::ComInterface::cast::<IVpnNativeProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProfileName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProfileName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppTriggers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Routes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainNameInfoList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrafficFilters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RememberCredentials(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RememberCredentials)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRememberCredentials)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysOn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysOn)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for VpnNativeProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnNativeProfile {}
impl ::core::fmt::Debug for VpnNativeProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnNativeProfile").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnNativeProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnNativeProfile;{a4aee29e-6417-4333-9842-f0a66db69802})");
}
impl ::core::clone::Clone for VpnNativeProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnNativeProfile {
    type Vtable = IVpnNativeProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnNativeProfile {
    const IID: ::windows_core::GUID = <IVpnNativeProfile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnNativeProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnNativeProfile";
}
::windows_core::imp::interface_hierarchy!(VpnNativeProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnProfile> for VpnNativeProfile {}
unsafe impl ::core::marker::Send for VpnNativeProfile {}
unsafe impl ::core::marker::Sync for VpnNativeProfile {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnPacketBuffer(::windows_core::IUnknown);
impl VpnPacketBuffer {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<VpnPacketBufferStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTransportAffinity(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTransportAffinity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransportAffinity(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransportAffinity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<VpnAppId> {
        let this = &::windows_core::ComInterface::cast::<IVpnPacketBuffer2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTransportContext<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<IVpnPacketBuffer3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTransportContext)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TransportContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IVpnPacketBuffer3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TransportContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateVpnPacketBuffer<P0>(parentbuffer: P0, offset: u32, length: u32) -> ::windows_core::Result<VpnPacketBuffer>
    where
        P0: ::windows_core::IntoParam<VpnPacketBuffer>,
    {
        Self::IVpnPacketBufferFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnPacketBuffer)(::windows_core::Interface::as_raw(this), parentbuffer.into_param().abi(), offset, length, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVpnPacketBufferFactory<R, F: FnOnce(&IVpnPacketBufferFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnPacketBuffer, IVpnPacketBufferFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnPacketBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnPacketBuffer {}
impl ::core::fmt::Debug for VpnPacketBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnPacketBuffer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnPacketBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPacketBuffer;{c2f891fc-4d5c-4a63-b70d-4e307eacce55})");
}
impl ::core::clone::Clone for VpnPacketBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnPacketBuffer {
    type Vtable = IVpnPacketBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnPacketBuffer {
    const IID: ::windows_core::GUID = <IVpnPacketBuffer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnPacketBuffer {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPacketBuffer";
}
::windows_core::imp::interface_hierarchy!(VpnPacketBuffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnPacketBuffer {}
unsafe impl ::core::marker::Sync for VpnPacketBuffer {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnPacketBufferList(::windows_core::IUnknown);
impl VpnPacketBufferList {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<VpnPacketBuffer>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<VpnPacketBuffer>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Append<P0>(&self, nextvpnpacketbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnPacketBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), nextvpnpacketbuffer.into_param().abi()).ok() }
    }
    pub fn AddAtBegin<P0>(&self, nextvpnpacketbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnPacketBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddAtBegin)(::windows_core::Interface::as_raw(this), nextvpnpacketbuffer.into_param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<VpnPacketBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveAtBegin(&self) -> ::windows_core::Result<VpnPacketBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAtBegin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStatus)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<VpnPacketBufferStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnPacketBufferList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnPacketBufferList {}
impl ::core::fmt::Debug for VpnPacketBufferList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnPacketBufferList").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnPacketBufferList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPacketBufferList;{c2f891fc-4d5c-4a63-b70d-4e307eacce77})");
}
impl ::core::clone::Clone for VpnPacketBufferList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnPacketBufferList {
    type Vtable = IVpnPacketBufferList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnPacketBufferList {
    const IID: ::windows_core::GUID = <IVpnPacketBufferList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnPacketBufferList {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPacketBufferList";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for VpnPacketBufferList {
    type Item = VpnPacketBuffer;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &VpnPacketBufferList {
    type Item = VpnPacketBuffer;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(VpnPacketBufferList, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<VpnPacketBuffer>> for VpnPacketBufferList {}
unsafe impl ::core::marker::Send for VpnPacketBufferList {}
unsafe impl ::core::marker::Sync for VpnPacketBufferList {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnPickedCredential(::windows_core::IUnknown);
impl VpnPickedCredential {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn PasskeyCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PasskeyCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AdditionalPin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdditionalPin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn OldPasswordCredential(&self) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldPasswordCredential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnPickedCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnPickedCredential {}
impl ::core::fmt::Debug for VpnPickedCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnPickedCredential").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnPickedCredential {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPickedCredential;{9a793ac7-8854-4e52-ad97-24dd9a842bce})");
}
impl ::core::clone::Clone for VpnPickedCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnPickedCredential {
    type Vtable = IVpnPickedCredential_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnPickedCredential {
    const IID: ::windows_core::GUID = <IVpnPickedCredential as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnPickedCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPickedCredential";
}
::windows_core::imp::interface_hierarchy!(VpnPickedCredential, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnPickedCredential {}
unsafe impl ::core::marker::Sync for VpnPickedCredential {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnPlugInProfile(::windows_core::IUnknown);
impl VpnPlugInProfile {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnPlugInProfile, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ServerUris(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServerUris)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CustomConfiguration(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomConfiguration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCustomConfiguration(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCustomConfiguration)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn VpnPluginPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VpnPluginPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVpnPluginPackageFamilyName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVpnPluginPackageFamilyName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RequireVpnClientAppUI(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequireVpnClientAppUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRequireVpnClientAppUI)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ConnectionStatus(&self) -> ::windows_core::Result<VpnManagementConnectionStatus> {
        let this = &::windows_core::ComInterface::cast::<IVpnPlugInProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProfileName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProfileName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppTriggers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnAppId>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppTriggers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Routes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Routes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DomainNameInfoList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DomainNameInfoList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilters(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrafficFilters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RememberCredentials(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RememberCredentials)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRememberCredentials(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRememberCredentials)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AlwaysOn(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AlwaysOn)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlwaysOn(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVpnProfile>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAlwaysOn)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for VpnPlugInProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnPlugInProfile {}
impl ::core::fmt::Debug for VpnPlugInProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnPlugInProfile").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnPlugInProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnPlugInProfile;{0edf0da4-4f00-4589-8d7b-4bf988f6542c})");
}
impl ::core::clone::Clone for VpnPlugInProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnPlugInProfile {
    type Vtable = IVpnPlugInProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnPlugInProfile {
    const IID: ::windows_core::GUID = <IVpnPlugInProfile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnPlugInProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnPlugInProfile";
}
::windows_core::imp::interface_hierarchy!(VpnPlugInProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IVpnProfile> for VpnPlugInProfile {}
unsafe impl ::core::marker::Send for VpnPlugInProfile {}
unsafe impl ::core::marker::Sync for VpnPlugInProfile {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnRoute(::windows_core::IUnknown);
impl VpnRoute {
    pub fn SetAddress<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::HostName>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAddress)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Address(&self) -> ::windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPrefixSize(&self, value: u8) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPrefixSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PrefixSize(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrefixSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateVpnRoute<P0>(address: P0, prefixsize: u8) -> ::windows_core::Result<VpnRoute>
    where
        P0: ::windows_core::IntoParam<super::HostName>,
    {
        Self::IVpnRouteFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVpnRoute)(::windows_core::Interface::as_raw(this), address.into_param().abi(), prefixsize, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVpnRouteFactory<R, F: FnOnce(&IVpnRouteFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnRoute, IVpnRouteFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnRoute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnRoute {}
impl ::core::fmt::Debug for VpnRoute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnRoute").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnRoute {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnRoute;{b5731b83-0969-4699-938e-7776db29cfb3})");
}
impl ::core::clone::Clone for VpnRoute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnRoute {
    type Vtable = IVpnRoute_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnRoute {
    const IID: ::windows_core::GUID = <IVpnRoute as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnRoute {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnRoute";
}
::windows_core::imp::interface_hierarchy!(VpnRoute, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnRoute {}
unsafe impl ::core::marker::Sync for VpnRoute {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnRouteAssignment(::windows_core::IUnknown);
impl VpnRouteAssignment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnRouteAssignment, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv4InclusionRoutes<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIpv4InclusionRoutes)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv6InclusionRoutes<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIpv6InclusionRoutes)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv4InclusionRoutes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ipv4InclusionRoutes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv6InclusionRoutes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ipv6InclusionRoutes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv4ExclusionRoutes<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIpv4ExclusionRoutes)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetIpv6ExclusionRoutes<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IVector<VpnRoute>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIpv6ExclusionRoutes)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv4ExclusionRoutes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ipv4ExclusionRoutes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Ipv6ExclusionRoutes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnRoute>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ipv6ExclusionRoutes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExcludeLocalSubnets(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExcludeLocalSubnets)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExcludeLocalSubnets(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExcludeLocalSubnets)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnRouteAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnRouteAssignment {}
impl ::core::fmt::Debug for VpnRouteAssignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnRouteAssignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnRouteAssignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnRouteAssignment;{db64de22-ce39-4a76-9550-f61039f80e48})");
}
impl ::core::clone::Clone for VpnRouteAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnRouteAssignment {
    type Vtable = IVpnRouteAssignment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnRouteAssignment {
    const IID: ::windows_core::GUID = <IVpnRouteAssignment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnRouteAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnRouteAssignment";
}
::windows_core::imp::interface_hierarchy!(VpnRouteAssignment, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnRouteAssignment {}
unsafe impl ::core::marker::Sync for VpnRouteAssignment {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnSystemHealth(::windows_core::IUnknown);
impl VpnSystemHealth {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn StatementOfHealth(&self) -> ::windows_core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatementOfHealth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VpnSystemHealth {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnSystemHealth {}
impl ::core::fmt::Debug for VpnSystemHealth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnSystemHealth").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnSystemHealth {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnSystemHealth;{99a8f8af-c0ee-4e75-817a-f231aee5123d})");
}
impl ::core::clone::Clone for VpnSystemHealth {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnSystemHealth {
    type Vtable = IVpnSystemHealth_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnSystemHealth {
    const IID: ::windows_core::GUID = <IVpnSystemHealth as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnSystemHealth {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnSystemHealth";
}
::windows_core::imp::interface_hierarchy!(VpnSystemHealth, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnSystemHealth {}
unsafe impl ::core::marker::Sync for VpnSystemHealth {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnTrafficFilter(::windows_core::IUnknown);
impl VpnTrafficFilter {
    pub fn AppId(&self) -> ::windows_core::Result<VpnAppId> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppId<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VpnAppId>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppClaims(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppClaims)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protocol(&self) -> ::windows_core::Result<VpnIPProtocol> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Protocol)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProtocol(&self, value: VpnIPProtocol) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProtocol)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalPortRanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalPortRanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemotePortRanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemotePortRanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalAddressRanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalAddressRanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoteAddressRanges(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoteAddressRanges)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoutingPolicyType(&self) -> ::windows_core::Result<VpnRoutingPolicyType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoutingPolicyType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRoutingPolicyType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create<P0>(appid: P0) -> ::windows_core::Result<VpnTrafficFilter>
    where
        P0: ::windows_core::IntoParam<VpnAppId>,
    {
        Self::IVpnTrafficFilterFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), appid.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVpnTrafficFilterFactory<R, F: FnOnce(&IVpnTrafficFilterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnTrafficFilter, IVpnTrafficFilterFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VpnTrafficFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnTrafficFilter {}
impl ::core::fmt::Debug for VpnTrafficFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnTrafficFilter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnTrafficFilter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnTrafficFilter;{2f691b60-6c9f-47f5-ac36-bb1b042e2c50})");
}
impl ::core::clone::Clone for VpnTrafficFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnTrafficFilter {
    type Vtable = IVpnTrafficFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnTrafficFilter {
    const IID: ::windows_core::GUID = <IVpnTrafficFilter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnTrafficFilter {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnTrafficFilter";
}
::windows_core::imp::interface_hierarchy!(VpnTrafficFilter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnTrafficFilter {}
unsafe impl ::core::marker::Sync for VpnTrafficFilter {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
pub struct VpnTrafficFilterAssignment(::windows_core::IUnknown);
impl VpnTrafficFilterAssignment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VpnTrafficFilterAssignment, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TrafficFilterList(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrafficFilterList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowOutbound(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowOutbound)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowOutbound(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowOutbound)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowInbound(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowInbound)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowInbound(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowInbound)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for VpnTrafficFilterAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VpnTrafficFilterAssignment {}
impl ::core::fmt::Debug for VpnTrafficFilterAssignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnTrafficFilterAssignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnTrafficFilterAssignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Vpn.VpnTrafficFilterAssignment;{56ccd45c-e664-471e-89cd-601603b9e0f3})");
}
impl ::core::clone::Clone for VpnTrafficFilterAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VpnTrafficFilterAssignment {
    type Vtable = IVpnTrafficFilterAssignment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VpnTrafficFilterAssignment {
    const IID: ::windows_core::GUID = <IVpnTrafficFilterAssignment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VpnTrafficFilterAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.VpnTrafficFilterAssignment";
}
::windows_core::imp::interface_hierarchy!(VpnTrafficFilterAssignment, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VpnTrafficFilterAssignment {}
unsafe impl ::core::marker::Sync for VpnTrafficFilterAssignment {}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnAppIdType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnAppIdType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnAppIdType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnAppIdType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnAppIdType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnAppIdType;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnAuthenticationMethod {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnAuthenticationMethod {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnAuthenticationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnAuthenticationMethod").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnAuthenticationMethod {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnAuthenticationMethod;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnChannelActivityEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnChannelActivityEventType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnChannelActivityEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnChannelActivityEventType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnChannelActivityEventType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnChannelActivityEventType;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnChannelRequestCredentialsOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnChannelRequestCredentialsOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnChannelRequestCredentialsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnChannelRequestCredentialsOptions").field(&self.0).finish()
    }
}
impl VpnChannelRequestCredentialsOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VpnChannelRequestCredentialsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VpnChannelRequestCredentialsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VpnChannelRequestCredentialsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for VpnChannelRequestCredentialsOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnChannelRequestCredentialsOptions;u4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnCredentialType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnCredentialType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnCredentialType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnCredentialType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnCredentialType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnCredentialType;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnDataPathType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnDataPathType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnDataPathType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnDataPathType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnDataPathType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnDataPathType;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnDomainNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnDomainNameType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnDomainNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnDomainNameType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnDomainNameType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnDomainNameType;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnIPProtocol {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnIPProtocol {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnIPProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnIPProtocol").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnIPProtocol {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnIPProtocol;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnManagementConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnManagementConnectionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnManagementConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnManagementConnectionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnManagementConnectionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnManagementConnectionStatus;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnManagementErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnManagementErrorStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnManagementErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnManagementErrorStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnManagementErrorStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnManagementErrorStatus;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnNativeProtocolType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnNativeProtocolType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnNativeProtocolType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnNativeProtocolType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnNativeProtocolType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnNativeProtocolType;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnPacketBufferStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnPacketBufferStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnPacketBufferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnPacketBufferStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnPacketBufferStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnPacketBufferStatus;i4)");
}
#[doc = "*Required features: `\"Networking_Vpn\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for VpnRoutingPolicyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VpnRoutingPolicyType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VpnRoutingPolicyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VpnRoutingPolicyType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VpnRoutingPolicyType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Vpn.VpnRoutingPolicyType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
