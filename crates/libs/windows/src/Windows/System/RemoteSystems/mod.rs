#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownRemoteSystemCapabilitiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IKnownRemoteSystemCapabilitiesStatics {
    type Vtable = IKnownRemoteSystemCapabilitiesStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IKnownRemoteSystemCapabilitiesStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8108e380_7f8a_44e4_92cd_03b6469b94a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRemoteSystemCapabilitiesStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LaunchUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoteSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SpatialEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystem {
    type Vtable = IRemoteSystem_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed5838cd_1e10_4a8c_b4a6_4e5fd6f97721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemStatus) -> ::windows::core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystem2 {
    type Vtable = IRemoteSystem2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09dfe4ec_fb8b_4a08_a758_6876435d769e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitySupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitySupportedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystem3 {
    type Vtable = IRemoteSystem3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystem3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b4b495_b7c6_40be_831b_73562f12ffa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ManufacturerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystem4 {
    type Vtable = IRemoteSystem4_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystem4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf164ffe5_b987_4ca5_9926_fa0438be6273);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Platform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemPlatform) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystem5 {
    type Vtable = IRemoteSystem5_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystem5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb2ad723_e5e2_4ae2_a7a7_a1097a098e90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Apps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Apps: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystem6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystem6 {
    type Vtable = IRemoteSystem6_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystem6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4cda942_c027_533e_9384_3a19b4f7eef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemAddedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f39560f_e534_4697_8836_7abea151516e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemApp(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemApp {
    type Vtable = IRemoteSystemApp_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemApp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e5bcbd_d54d_41b1_9b16_6810a871ed4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemApp2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemApp2 {
    type Vtable = IRemoteSystemApp2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemApp2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6369bf15_0a96_577a_8ff6_c35904dfa8f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAppRegistration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemAppRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb47947b5_7035_4a5a_b8df_962d8f8431f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attributes: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAppRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemAppRegistrationStatics {
    type Vtable = IRemoteSystemAppRegistrationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemAppRegistrationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b99840_cfd2_453f_ae25_c2539f086afd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistrationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAuthorizationKindFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemAuthorizationKindFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b0dde8e_04d0_40f4_a27f_c2acbbd6b734);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystemAuthorizationKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemAuthorizationKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemAuthorizationKindFilterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemAuthorizationKindFilterFactory {
    type Vtable = IRemoteSystemAuthorizationKindFilterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemAuthorizationKindFilterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad65df4d_b66a_45a4_8177_8caed75d9e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemauthorizationkind: RemoteSystemAuthorizationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23278bc3_0d09_52cb_9c6a_eed2940bee43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsProximal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionInfoStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionInfoStatics {
    type Vtable = IRemoteSystemConnectionInfoStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionInfoStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac831e2d_66c5_56d7_a4ce_705d94925ad6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfoStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub TryCreateFromAppServiceConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    TryCreateFromAppServiceConnection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84ed4104_8d5e_4d72_8238_7621576c7a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionRequest2 {
    type Vtable = IRemoteSystemConnectionRequest2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionRequest2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12df6d6f_bffc_483a_8abe_d34a6c19f92b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystemApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequest3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionRequest3 {
    type Vtable = IRemoteSystemConnectionRequest3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionRequest3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde86c3e7_c9cc_5a50_b8d9_ba7b34bb8d0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionRequestFactory {
    type Vtable = IRemoteSystemConnectionRequestFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionRequestFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa0a0a20_baeb_4575_b530_810bb9786334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequestStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionRequestStatics {
    type Vtable = IRemoteSystemConnectionRequestStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionRequestStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86ca143d_8214_425c_8932_db49032d1306);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemapp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemConnectionRequestStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemConnectionRequestStatics2 {
    type Vtable = IRemoteSystemConnectionRequestStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemConnectionRequestStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x460f1027_64ec_598e_a800_4f2ee58def19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFromConnectionToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontoken: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFromConnectionTokenForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, connectiontoken: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemDiscoveryTypeFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemDiscoveryTypeFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42d9041f_ee5a_43da_ac6a_6fee25460741);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystemDiscoveryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemDiscoveryType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemDiscoveryTypeFilterFactory {
    type Vtable = IRemoteSystemDiscoveryTypeFilterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemDiscoveryTypeFilterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f9eb993_c260_4161_92f2_9c021f23fe5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discoverytype: RemoteSystemDiscoveryType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemEnumerationCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemEnumerationCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e83d5f_4030_4354_a060_14f1b22c545d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemEnumerationCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct IRemoteSystemFilter(::windows::core::IUnknown);
impl IRemoteSystemFilter {}
::windows::core::interface_hierarchy!(IRemoteSystemFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IRemoteSystemFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRemoteSystemFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteSystemFilter {}
impl ::core::fmt::Debug for IRemoteSystemFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteSystemFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRemoteSystemFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4a3ba9e4-99eb-45eb-ba16-0367728ff374}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IRemoteSystemFilter {
    type Vtable = IRemoteSystemFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a3ba9e4_99eb_45eb_ba16_0367728ff374);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemKindFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38e1c9ec_22c3_4ef6_901a_bbb1c7aad4ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoteSystemKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoteSystemKinds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindFilterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemKindFilterFactory {
    type Vtable = IRemoteSystemKindFilterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemKindFilterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1fb18ee_99ea_40bc_9a39_c670aa804a28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemKindStatics {
    type Vtable = IRemoteSystemKindStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemKindStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6317633_ab14_41d0_9553_796aadb882db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Phone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Hub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Holographic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Desktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Xbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemKindStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemKindStatics2 {
    type Vtable = IRemoteSystemKindStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemKindStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9e3a3d0_0466_4749_91e8_65f9d19a96a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Iot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Tablet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Laptop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemRemovedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b3d16bb_7306_49ea_b7df_67d5714cb013);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSession {
    type Vtable = IRemoteSystemSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69476a01_9ada_490f_9549_d31cb14c9e95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnected: usize,
    pub CreateParticipantWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendInvitationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invitee: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendInvitationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionAddedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd585d754_bc97_4c39_99b4_beca76e04c3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe48b2dd2_6820_4867_b425_d89c0a3ef7ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub JoinRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JoinRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJoinRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJoinRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveParticipantAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparticipant: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveParticipantAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSessionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionControllerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionControllerFactory {
    type Vtable = IRemoteSystemSessionControllerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionControllerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfcc2f6b_ac3d_4199_82cd_6670a773ef2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionControllerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateControllerWithSessionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionCreationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionCreationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa79812c2_37de_448c_8b83_a30aa3c4ead6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionCreationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionCreationStatus) -> ::windows::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionDisconnectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionDisconnectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde0bc69b_77c5_461c_8209_7c6c5d3111ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionDisconnectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionDisconnectedReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff4df648_8b0a_4e9a_9905_69e4b841c588);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub JoinAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JoinAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInvitation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionInvitation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e32cc91_51d7_4766_a121_25516c3b8294);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInvitationListener(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionInvitationListener {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08f4003f_bc71_49e1_874a_31ddff9a27b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationListener_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InvitationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvitationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvitationReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvitationReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionInvitationReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e964a2d_a10d_4edb_8dea_54d20ac19543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Invitation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionJoinRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionJoinRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20600068_7994_4331_86d1_d89d882585ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionJoinRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdbca4fc3_82b9_4816_9c24_e40e61774bd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub JoinRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionJoinResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionJoinResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce7b1f04_a03e_41a4_900b_1e79328c1267);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionJoinStatus) -> ::windows::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionMessageChannel(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionMessageChannel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9524d12a_73d9_4c10_b751_c26784437127);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BroadcastValueSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagedata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BroadcastValueSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendValueSetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagedata: *mut ::core::ffi::c_void, participant: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendValueSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendValueSetToParticipantsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagedata: *mut ::core::ffi::c_void, participants: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendValueSetToParticipantsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ValueSetReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueSetReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueSetReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueSetReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionMessageChannelFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionMessageChannelFactory {
    type Vtable = IRemoteSystemSessionMessageChannelFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionMessageChannelFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x295e1c4a_bd16_4298_b7ce_415482b0e11d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannelFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, channelname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithReliability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, channelname: *mut ::core::ffi::c_void, reliability: RemoteSystemSessionMessageChannelReliability, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x740ed755_8418_4f01_9353_e21c9ecc6cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsInviteOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsInviteOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipant(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionParticipant {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e90058c_acf9_4729_8a17_44e7baed5dcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipant_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetHostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetHostNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionParticipantAddedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd35a57d8_c9a1_4bb7_b6b0_79bb91adf93d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionParticipantRemovedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x866ef088_de68_4abf_88a1_f90d16274192);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Participant: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionParticipantWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionParticipantWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdd02cc_aa87_4d79_b6cc_4459b3e92075);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionParticipantWatcherStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionRemovedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf82914e_39a1_4dea_9d63_43798d5bbbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionStatics {
    type Vtable = IRemoteSystemSessionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8524899f_fd20_44e3_9565_e75a3b14c66e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16875069_231e_4c91_8ec8_b3a39d9e55a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionValueSetReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06f31785_2da5_4e58_a78f_9e8d0784ee25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemSessionWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemSessionWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8003e340_0c41_4a62_b6d7_bdbe2b19be2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionWatcherStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemStatics {
    type Vtable = IRemoteSystemStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa485b392_ff2b_4b47_be62_743f2f140f30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub FindByHostNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))]
    FindByHostNameAsync: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithFilters: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemStatics2 {
    type Vtable = IRemoteSystemStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c98edca_6f99_4c52_a272_ea4f36471744);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsAuthorizationKindEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: RemoteSystemAuthorizationKind, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemStatics3 {
    type Vtable = IRemoteSystemStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9995f16f_0b3c_5ac5_b325_cc73f437dfcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWatcherForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithFiltersForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, filters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithFiltersForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatusTypeFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemStatusTypeFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c39514e_cbb6_4777_8534_2e0c521affa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystemStatusType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemStatusType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemStatusTypeFilterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemStatusTypeFilterFactory {
    type Vtable = IRemoteSystemStatusTypeFilterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemStatusTypeFilterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cf78fa_d724_4125_ac7a_8d281e44c949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotesystemstatustype: RemoteSystemStatusType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemUpdatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7502ff0e_dbcb_4155_b4ca_b30a04f27627);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemUpdatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d600c7e_2c07_48c5_889c_455d2b099771);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoteSystemAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteSystemAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteSystemAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteSystemAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoteSystemUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteSystemUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteSystemUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteSystemUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoteSystemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteSystemRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteSystemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteSystemRemoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcher2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemWatcher2 {
    type Vtable = IRemoteSystemWatcher2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemWatcher2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73436700_19ca_48f9_a4cd_780f7ad58c71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcher3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemWatcher3 {
    type Vtable = IRemoteSystemWatcher3_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemWatcher3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf79c0fcf_a913_55d3_8413_418fcf15ba54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemWatcherErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74c5c6af_5114_4426_9216_20d81f8519ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemWatcherError) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWebAccountFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemWebAccountFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fb75873_87c8_5d8f_977e_f69f96d67238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteSystemWebAccountFilterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteSystemWebAccountFilterFactory {
    type Vtable = IRemoteSystemWebAccountFilterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteSystemWebAccountFilterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x348a2709_5f4d_5127_b4a7_bf99d5252b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
pub struct KnownRemoteSystemCapabilities;
impl KnownRemoteSystemCapabilities {
    pub fn AppService() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AppService)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn LaunchUri() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn RemoteSession() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSession)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SpatialEntity() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SpatialEntity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownRemoteSystemCapabilitiesStatics<R, F: FnOnce(&IKnownRemoteSystemCapabilitiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KnownRemoteSystemCapabilities, IKnownRemoteSystemCapabilitiesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KnownRemoteSystemCapabilities {
    const NAME: &'static str = "Windows.System.RemoteSystems.KnownRemoteSystemCapabilities";
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystem(::windows::core::IUnknown);
impl RemoteSystem {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<RemoteSystemStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAvailableByProximity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAvailableByProximity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAvailableBySpatialProximity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitySupportedAsync(&self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCapabilitySupportedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(capabilityname), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ManufacturerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManufacturerDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ModelDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRemoteSystem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Platform(&self) -> ::windows::core::Result<RemoteSystemPlatform> {
        let this = &::windows::core::Interface::cast::<IRemoteSystem4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Platform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Apps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>> {
        let this = &::windows::core::Interface::cast::<IRemoteSystem5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Apps)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = &::windows::core::Interface::cast::<IRemoteSystem6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub fn FindByHostNameAsync(hostname: &super::super::Networking::HostName) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystem>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindByHostNameAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(hostname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithFilters<P0, E0>(filters: P0) -> ::windows::core::Result<RemoteSystemWatcher>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcherWithFilters)(::windows::core::Vtable::as_raw(this), filters.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsAuthorizationKindEnabled(kind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<bool> {
        Self::IRemoteSystemStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAuthorizationKindEnabled)(::windows::core::Vtable::as_raw(this), kind, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWatcherForUser(user: &super::User) -> ::windows::core::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcherForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithFiltersForUser<P0, E0>(user: &super::User, filters: P0) -> ::windows::core::Result<RemoteSystemWatcher>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcherWithFiltersForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), filters.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatics<R, F: FnOnce(&IRemoteSystemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystem, IRemoteSystemStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatics2<R, F: FnOnce(&IRemoteSystemStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystem, IRemoteSystemStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatics3<R, F: FnOnce(&IRemoteSystemStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystem, IRemoteSystemStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystem {}
impl ::core::fmt::Debug for RemoteSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystem;{ed5838cd-1e10-4a8c-b4a6-4e5fd6f97721})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystem {
    type Vtable = IRemoteSystem_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystem {
    const IID: ::windows::core::GUID = <IRemoteSystem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystem {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystem";
}
::windows::core::interface_hierarchy!(RemoteSystem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystem {}
unsafe impl ::core::marker::Sync for RemoteSystem {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemAddedEventArgs(::windows::core::IUnknown);
impl RemoteSystemAddedEventArgs {
    pub fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemAddedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAddedEventArgs;{8f39560f-e534-4697-8836-7abea151516e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemAddedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemAddedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAddedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemAddedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemAddedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemApp(::windows::core::IUnknown);
impl RemoteSystemApp {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAvailableByProximity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAvailableByProximity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAvailableBySpatialProximity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attributes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectionToken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemApp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemApp {}
impl ::core::fmt::Debug for RemoteSystemApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemApp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemApp {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemApp;{80e5bcbd-d54d-41b1-9b16-6810a871ed4f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemApp {
    type Vtable = IRemoteSystemApp_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemApp {
    const IID: ::windows::core::GUID = <IRemoteSystemApp as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemApp {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemApp";
}
::windows::core::interface_hierarchy!(RemoteSystemApp, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemApp {}
unsafe impl ::core::marker::Sync for RemoteSystemApp {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemAppRegistration(::windows::core::IUnknown);
impl RemoteSystemAppRegistration {
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attributes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attributes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetForUser(user: &super::User) -> ::windows::core::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemAppRegistrationStatics<R, F: FnOnce(&IRemoteSystemAppRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemAppRegistration, IRemoteSystemAppRegistrationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemAppRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemAppRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemAppRegistration {}
impl ::core::fmt::Debug for RemoteSystemAppRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAppRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemAppRegistration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAppRegistration;{b47947b5-7035-4a5a-b8df-962d8f8431f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemAppRegistration {
    const IID: ::windows::core::GUID = <IRemoteSystemAppRegistration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemAppRegistration {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAppRegistration";
}
::windows::core::interface_hierarchy!(RemoteSystemAppRegistration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemAppRegistration {}
unsafe impl ::core::marker::Sync for RemoteSystemAppRegistration {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemAuthorizationKindFilter(::windows::core::IUnknown);
impl RemoteSystemAuthorizationKindFilter {
    pub fn RemoteSystemAuthorizationKind(&self) -> ::windows::core::Result<RemoteSystemAuthorizationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemAuthorizationKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(remotesystemauthorizationkind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<RemoteSystemAuthorizationKindFilter> {
        Self::IRemoteSystemAuthorizationKindFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), remotesystemauthorizationkind, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemAuthorizationKindFilterFactory<R, F: FnOnce(&IRemoteSystemAuthorizationKindFilterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemAuthorizationKindFilter, IRemoteSystemAuthorizationKindFilterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemAuthorizationKindFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemAuthorizationKindFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemAuthorizationKindFilter {}
impl ::core::fmt::Debug for RemoteSystemAuthorizationKindFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAuthorizationKindFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemAuthorizationKindFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter;{6b0dde8e-04d0-40f4-a27f-c2acbbd6b734})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemAuthorizationKindFilter {
    const IID: ::windows::core::GUID = <IRemoteSystemAuthorizationKindFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemAuthorizationKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter";
}
::windows::core::interface_hierarchy!(RemoteSystemAuthorizationKindFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: RemoteSystemAuthorizationKindFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemAuthorizationKindFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemAuthorizationKindFilter> for ::windows::core::InParam<IRemoteSystemFilter> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemAuthorizationKindFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RemoteSystemAuthorizationKindFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemAuthorizationKindFilter {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemConnectionInfo(::windows::core::IUnknown);
impl RemoteSystemConnectionInfo {
    pub fn IsProximal(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsProximal)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn TryCreateFromAppServiceConnection(connection: &super::super::ApplicationModel::AppService::AppServiceConnection) -> ::windows::core::Result<RemoteSystemConnectionInfo> {
        Self::IRemoteSystemConnectionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCreateFromAppServiceConnection)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(connection), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionInfoStatics<R, F: FnOnce(&IRemoteSystemConnectionInfoStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemConnectionInfo, IRemoteSystemConnectionInfoStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemConnectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemConnectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemConnectionInfo {}
impl ::core::fmt::Debug for RemoteSystemConnectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemConnectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemConnectionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionInfo;{23278bc3-0d09-52cb-9c6a-eed2940bee43})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemConnectionInfo {
    const IID: ::windows::core::GUID = <IRemoteSystemConnectionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionInfo";
}
::windows::core::interface_hierarchy!(RemoteSystemConnectionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemConnectionInfo {}
unsafe impl ::core::marker::Sync for RemoteSystemConnectionInfo {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemConnectionRequest(::windows::core::IUnknown);
impl RemoteSystemConnectionRequest {
    pub fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoteSystemApp(&self) -> ::windows::core::Result<RemoteSystemApp> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemConnectionRequest2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemApp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemConnectionRequest3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectionToken)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(remotesystem: &RemoteSystem) -> ::windows::core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remotesystem), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateForApp(remotesystemapp: &RemoteSystemApp) -> ::windows::core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForApp)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(remotesystemapp), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromConnectionToken(connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromConnectionToken)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(connectiontoken), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateFromConnectionTokenForUser(user: &super::User, connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromConnectionTokenForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(connectiontoken), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionRequestFactory<R, F: FnOnce(&IRemoteSystemConnectionRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionRequestStatics<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemConnectionRequestStatics2<R, F: FnOnce(&IRemoteSystemConnectionRequestStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemConnectionRequest, IRemoteSystemConnectionRequestStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemConnectionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemConnectionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemConnectionRequest {}
impl ::core::fmt::Debug for RemoteSystemConnectionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemConnectionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemConnectionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionRequest;{84ed4104-8d5e-4d72-8238-7621576c7a67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemConnectionRequest {
    const IID: ::windows::core::GUID = <IRemoteSystemConnectionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemConnectionRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionRequest";
}
::windows::core::interface_hierarchy!(RemoteSystemConnectionRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemConnectionRequest {}
unsafe impl ::core::marker::Sync for RemoteSystemConnectionRequest {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemDiscoveryTypeFilter(::windows::core::IUnknown);
impl RemoteSystemDiscoveryTypeFilter {
    pub fn RemoteSystemDiscoveryType(&self) -> ::windows::core::Result<RemoteSystemDiscoveryType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemDiscoveryType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(discoverytype: RemoteSystemDiscoveryType) -> ::windows::core::Result<RemoteSystemDiscoveryTypeFilter> {
        Self::IRemoteSystemDiscoveryTypeFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), discoverytype, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemDiscoveryTypeFilterFactory<R, F: FnOnce(&IRemoteSystemDiscoveryTypeFilterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemDiscoveryTypeFilter, IRemoteSystemDiscoveryTypeFilterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemDiscoveryTypeFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemDiscoveryTypeFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemDiscoveryTypeFilter {}
impl ::core::fmt::Debug for RemoteSystemDiscoveryTypeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemDiscoveryTypeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemDiscoveryTypeFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter;{42d9041f-ee5a-43da-ac6a-6fee25460741})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemDiscoveryTypeFilter {
    const IID: ::windows::core::GUID = <IRemoteSystemDiscoveryTypeFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemDiscoveryTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter";
}
::windows::core::interface_hierarchy!(RemoteSystemDiscoveryTypeFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: RemoteSystemDiscoveryTypeFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemDiscoveryTypeFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemDiscoveryTypeFilter> for ::windows::core::InParam<IRemoteSystemFilter> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemDiscoveryTypeFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RemoteSystemDiscoveryTypeFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemDiscoveryTypeFilter {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemEnumerationCompletedEventArgs(::windows::core::IUnknown);
impl RemoteSystemEnumerationCompletedEventArgs {}
impl ::core::clone::Clone for RemoteSystemEnumerationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemEnumerationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemEnumerationCompletedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemEnumerationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemEnumerationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemEnumerationCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs;{c6e83d5f-4030-4354-a060-14f1b22c545d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemEnumerationCompletedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemEnumerationCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemEnumerationCompletedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemEnumerationCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemEnumerationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemEnumerationCompletedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemKindFilter(::windows::core::IUnknown);
impl RemoteSystemKindFilter {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoteSystemKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemKinds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<P0, E0>(remotesystemkinds: P0) -> ::windows::core::Result<RemoteSystemKindFilter>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IRemoteSystemKindFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), remotesystemkinds.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemKindFilterFactory<R, F: FnOnce(&IRemoteSystemKindFilterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemKindFilter, IRemoteSystemKindFilterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemKindFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemKindFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemKindFilter {}
impl ::core::fmt::Debug for RemoteSystemKindFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemKindFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemKindFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemKindFilter;{38e1c9ec-22c3-4ef6-901a-bbb1c7aad4ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemKindFilter {
    const IID: ::windows::core::GUID = <IRemoteSystemKindFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKindFilter";
}
::windows::core::interface_hierarchy!(RemoteSystemKindFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: RemoteSystemKindFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemKindFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemKindFilter> for ::windows::core::InParam<IRemoteSystemFilter> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemKindFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RemoteSystemKindFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemKindFilter {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
pub struct RemoteSystemKinds;
impl RemoteSystemKinds {
    pub fn Phone() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Phone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Hub() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Hub)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Holographic() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Holographic)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Desktop() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Desktop)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Xbox() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Xbox)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Iot() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Iot)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Tablet() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Tablet)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Laptop() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Laptop)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemKindStatics<R, F: FnOnce(&IRemoteSystemKindStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRemoteSystemKindStatics2<R, F: FnOnce(&IRemoteSystemKindStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemKinds, IRemoteSystemKindStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for RemoteSystemKinds {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKinds";
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemRemovedEventArgs(::windows::core::IUnknown);
impl RemoteSystemRemovedEventArgs {
    pub fn RemoteSystemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemRemovedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs;{8b3d16bb-7306-49ea-b7df-67d5714cb013})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemRemovedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemRemovedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemRemovedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemRemovedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSession(::windows::core::IUnknown);
impl RemoteSystemSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ControllerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControllerDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Disconnected(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Disconnected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDisconnected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn CreateParticipantWatcher(&self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcher> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateParticipantWatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendInvitationAsync(&self, invitee: &RemoteSystem) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendInvitationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(invitee), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows::core::Result<RemoteSystemSessionWatcher> {
        Self::IRemoteSystemSessionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemSessionStatics<R, F: FnOnce(&IRemoteSystemSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemSession, IRemoteSystemSessionStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSession {}
impl ::core::fmt::Debug for RemoteSystemSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSession;{69476a01-9ada-490f-9549-d31cb14c9e95})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSession {
    type Vtable = IRemoteSystemSession_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSession {
    const IID: ::windows::core::GUID = <IRemoteSystemSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSession {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSession";
}
::windows::core::interface_hierarchy!(RemoteSystemSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<RemoteSystemSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: RemoteSystemSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RemoteSystemSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RemoteSystemSession> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RemoteSystemSession {}
unsafe impl ::core::marker::Sync for RemoteSystemSession {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionAddedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionAddedEventArgs {
    pub fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionAddedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs;{d585d754-bc97-4c39-99b4-beca76e04c3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionAddedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionAddedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionAddedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionAddedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionController(::windows::core::IUnknown);
impl RemoteSystemSessionController {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn JoinRequested(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JoinRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveJoinRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveJoinRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveParticipantAsync(&self, pparticipant: &RemoteSystemSessionParticipant) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoveParticipantAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pparticipant), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateSessionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSessionAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateController(displayname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(displayname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateControllerWithSessionOptions(displayname: &::windows::core::HSTRING, options: &RemoteSystemSessionOptions) -> ::windows::core::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateControllerWithSessionOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemSessionControllerFactory<R, F: FnOnce(&IRemoteSystemSessionControllerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemSessionController, IRemoteSystemSessionControllerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemSessionController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionController {}
impl ::core::fmt::Debug for RemoteSystemSessionController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionController;{e48b2dd2-6820-4867-b425-d89c0a3ef7ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionController {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionController {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionController";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionController {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionController {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionCreationResult(::windows::core::IUnknown);
impl RemoteSystemSessionCreationResult {
    pub fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionCreationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Session)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionCreationResult {}
impl ::core::fmt::Debug for RemoteSystemSessionCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionCreationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionCreationResult;{a79812c2-37de-448c-8b83-a30aa3c4ead6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionCreationResult {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionCreationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionCreationResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionCreationResult";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionCreationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionCreationResult {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionCreationResult {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionDisconnectedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionDisconnectedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<RemoteSystemSessionDisconnectedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionDisconnectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionDisconnectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionDisconnectedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionDisconnectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionDisconnectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionDisconnectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs;{de0bc69b-77c5-461c-8209-7c6c5d3111ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionDisconnectedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionDisconnectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionDisconnectedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionDisconnectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionDisconnectedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionDisconnectedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionInfo(::windows::core::IUnknown);
impl RemoteSystemSessionInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ControllerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControllerDisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn JoinAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JoinAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInfo {}
impl ::core::fmt::Debug for RemoteSystemSessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInfo;{ff4df648-8b0a-4e9a-9905-69e4b841c588})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionInfo {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInfo";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionInfo {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInfo {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionInvitation(::windows::core::IUnknown);
impl RemoteSystemSessionInvitation {
    pub fn Sender(&self) -> ::windows::core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Sender)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInvitation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInvitation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInvitation {}
impl ::core::fmt::Debug for RemoteSystemSessionInvitation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInvitation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionInvitation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitation;{3e32cc91-51d7-4766-a121-25516c3b8294})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionInvitation {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionInvitation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionInvitation {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitation";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionInvitation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitation {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitation {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionInvitationListener(::windows::core::IUnknown);
impl RemoteSystemSessionInvitationListener {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemSessionInvitationListener, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvitationReceived(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InvitationReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInvitationReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveInvitationReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInvitationListener {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInvitationListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInvitationListener {}
impl ::core::fmt::Debug for RemoteSystemSessionInvitationListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInvitationListener").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionInvitationListener {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener;{08f4003f-bc71-49e1-874a-31ddff9a27b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionInvitationListener {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionInvitationListener as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionInvitationListener {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionInvitationListener, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitationListener {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitationListener {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionInvitationReceivedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionInvitationReceivedEventArgs {
    pub fn Invitation(&self) -> ::windows::core::Result<RemoteSystemSessionInvitation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Invitation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionInvitationReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionInvitationReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionInvitationReceivedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionInvitationReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionInvitationReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionInvitationReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs;{5e964a2d-a10d-4edb-8dea-54d20ac19543})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionInvitationReceivedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionInvitationReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionInvitationReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionInvitationReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionInvitationReceivedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionInvitationReceivedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionJoinRequest(::windows::core::IUnknown);
impl RemoteSystemSessionJoinRequest {
    pub fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Participant)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Accept(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Accept)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionJoinRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionJoinRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionJoinRequest {}
impl ::core::fmt::Debug for RemoteSystemSessionJoinRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionJoinRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest;{20600068-7994-4331-86d1-d89d882585ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionJoinRequest {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionJoinRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionJoinRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionJoinRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinRequest {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinRequest {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionJoinRequestedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionJoinRequestedEventArgs {
    pub fn JoinRequest(&self) -> ::windows::core::Result<RemoteSystemSessionJoinRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JoinRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionJoinRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionJoinRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionJoinRequestedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionJoinRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionJoinRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs;{dbca4fc3-82b9-4816-9c24-e40e61774bd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionJoinRequestedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionJoinRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionJoinRequestedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionJoinRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinRequestedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinRequestedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionJoinResult(::windows::core::IUnknown);
impl RemoteSystemSessionJoinResult {
    pub fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionJoinStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Session)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionJoinResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionJoinResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionJoinResult {}
impl ::core::fmt::Debug for RemoteSystemSessionJoinResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionJoinResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinResult;{ce7b1f04-a03e-41a4-900b-1e79328c1267})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionJoinResult {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionJoinResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionJoinResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinResult";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionJoinResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionJoinResult {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionJoinResult {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionMessageChannel(::windows::core::IUnknown);
impl RemoteSystemSessionMessageChannel {
    pub fn Session(&self) -> ::windows::core::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Session)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BroadcastValueSetAsync(&self, messagedata: &super::super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BroadcastValueSetAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagedata), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendValueSetAsync(&self, messagedata: &super::super::Foundation::Collections::ValueSet, participant: &RemoteSystemSessionParticipant) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendValueSetAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagedata), ::core::mem::transmute_copy(participant), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SendValueSetToParticipantsAsync<P0, E0>(&self, messagedata: &super::super::Foundation::Collections::ValueSet, participants: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendValueSetToParticipantsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagedata), participants.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValueSetReceived(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueSetReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueSetReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveValueSetReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Create(session: &RemoteSystemSession, channelname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(session), ::core::mem::transmute_copy(channelname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithReliability(session: &RemoteSystemSession, channelname: &::windows::core::HSTRING, reliability: RemoteSystemSessionMessageChannelReliability) -> ::windows::core::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithReliability)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(session), ::core::mem::transmute_copy(channelname), reliability, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemSessionMessageChannelFactory<R, F: FnOnce(&IRemoteSystemSessionMessageChannelFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemSessionMessageChannel, IRemoteSystemSessionMessageChannelFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemSessionMessageChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionMessageChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionMessageChannel {}
impl ::core::fmt::Debug for RemoteSystemSessionMessageChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionMessageChannel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionMessageChannel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel;{9524d12a-73d9-4c10-b751-c26784437127})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionMessageChannel {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionMessageChannel as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionMessageChannel {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionMessageChannel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionMessageChannel {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionMessageChannel {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionOptions(::windows::core::IUnknown);
impl RemoteSystemSessionOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemSessionOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsInviteOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsInviteOnly)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsInviteOnly(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsInviteOnly)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionOptions {}
impl ::core::fmt::Debug for RemoteSystemSessionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionOptions;{740ed755-8418-4f01-9353-e21c9ecc6cfc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionOptions {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionOptions {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionOptions";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionOptions {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionOptions {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionParticipant(::windows::core::IUnknown);
impl RemoteSystemSessionParticipant {
    pub fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Networking\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetHostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetHostNames)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipant {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipant {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipant").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionParticipant {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipant;{7e90058c-acf9-4729-8a17-44e7baed5dcc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionParticipant {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionParticipant as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionParticipant {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipant";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionParticipant, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipant {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipant {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantAddedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionParticipantAddedEventArgs {
    pub fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Participant)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipantAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipantAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipantAddedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionParticipantAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs;{d35a57d8-c9a1-4bb7-b6b0-79bb91adf93d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionParticipantAddedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionParticipantAddedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionParticipantAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionParticipantAddedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantAddedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantAddedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantRemovedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionParticipantRemovedEventArgs {
    pub fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Participant)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipantRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipantRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipantRemovedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionParticipantRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs;{866ef088-de68-4abf-88a1-f90d16274192})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionParticipantRemovedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionParticipantRemovedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionParticipantRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionParticipantRemovedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantRemovedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantWatcher(::windows::core::IUnknown);
impl RemoteSystemSessionParticipantWatcher {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Added)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Removed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnumerationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEnumerationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionParticipantWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionParticipantWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionParticipantWatcher {}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionParticipantWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher;{dcdd02cc-aa87-4d79-b6cc-4459b3e92075})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionParticipantWatcher {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionParticipantWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionParticipantWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionParticipantWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionParticipantWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionParticipantWatcher {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionRemovedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionRemovedEventArgs {
    pub fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionRemovedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs;{af82914e-39a1-4dea-9d63-43798d5bbbd0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionRemovedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionRemovedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionRemovedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionRemovedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionRemovedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionUpdatedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionUpdatedEventArgs {
    pub fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionUpdatedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs;{16875069-231e-4c91-8ec8-b3a39d9e55a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionUpdatedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionValueSetReceivedEventArgs(::windows::core::IUnknown);
impl RemoteSystemSessionValueSetReceivedEventArgs {
    pub fn Sender(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Sender)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionValueSetReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionValueSetReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionValueSetReceivedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemSessionValueSetReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionValueSetReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionValueSetReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs;{06f31785-2da5-4e58-a78f-9e8d0784ee25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionValueSetReceivedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionValueSetReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionValueSetReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionValueSetReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionValueSetReceivedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionValueSetReceivedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionWatcher(::windows::core::IUnknown);
impl RemoteSystemSessionWatcher {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Added)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Updated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Removed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for RemoteSystemSessionWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemSessionWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemSessionWatcher {}
impl ::core::fmt::Debug for RemoteSystemSessionWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemSessionWatcher;{8003e340-0c41-4a62-b6d7-bdbe2b19be2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemSessionWatcher {
    const IID: ::windows::core::GUID = <IRemoteSystemSessionWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemSessionWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionWatcher";
}
::windows::core::interface_hierarchy!(RemoteSystemSessionWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemSessionWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemSessionWatcher {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemStatusTypeFilter(::windows::core::IUnknown);
impl RemoteSystemStatusTypeFilter {
    pub fn RemoteSystemStatusType(&self) -> ::windows::core::Result<RemoteSystemStatusType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemStatusType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(remotesystemstatustype: RemoteSystemStatusType) -> ::windows::core::Result<RemoteSystemStatusTypeFilter> {
        Self::IRemoteSystemStatusTypeFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), remotesystemstatustype, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemStatusTypeFilterFactory<R, F: FnOnce(&IRemoteSystemStatusTypeFilterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemStatusTypeFilter, IRemoteSystemStatusTypeFilterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemStatusTypeFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemStatusTypeFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemStatusTypeFilter {}
impl ::core::fmt::Debug for RemoteSystemStatusTypeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemStatusTypeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemStatusTypeFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter;{0c39514e-cbb6-4777-8534-2e0c521affa2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemStatusTypeFilter {
    const IID: ::windows::core::GUID = <IRemoteSystemStatusTypeFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemStatusTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter";
}
::windows::core::interface_hierarchy!(RemoteSystemStatusTypeFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: RemoteSystemStatusTypeFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemStatusTypeFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemStatusTypeFilter> for ::windows::core::InParam<IRemoteSystemFilter> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemStatusTypeFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RemoteSystemStatusTypeFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemStatusTypeFilter {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemUpdatedEventArgs(::windows::core::IUnknown);
impl RemoteSystemUpdatedEventArgs {
    pub fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemUpdatedEventArgs {}
impl ::core::fmt::Debug for RemoteSystemUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs;{7502ff0e-dbcb-4155-b4ca-b30a04f27627})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemUpdatedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemUpdatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemUpdatedEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemWatcher(::windows::core::IUnknown);
impl RemoteSystemWatcher {
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoteSystemAdded(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemAdded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteSystemAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoteSystemAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoteSystemUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteSystemUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoteSystemUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoteSystemRemoved(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteSystemRemoved)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteSystemRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoteSystemRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnumerationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEnumerationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred(&self, handler: &super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorOccurred)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveErrorOccurred)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn User(&self) -> ::windows::core::Result<super::User> {
        let this = &::windows::core::Interface::cast::<IRemoteSystemWatcher3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemWatcher {}
impl ::core::fmt::Debug for RemoteSystemWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWatcher;{5d600c7e-2c07-48c5-889c-455d2b099771})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemWatcher {
    const IID: ::windows::core::GUID = <IRemoteSystemWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcher";
}
::windows::core::interface_hierarchy!(RemoteSystemWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemWatcher {}
unsafe impl ::core::marker::Sync for RemoteSystemWatcher {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemWatcherErrorOccurredEventArgs(::windows::core::IUnknown);
impl RemoteSystemWatcherErrorOccurredEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<RemoteSystemWatcherError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteSystemWatcherErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemWatcherErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemWatcherErrorOccurredEventArgs {}
impl ::core::fmt::Debug for RemoteSystemWatcherErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWatcherErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemWatcherErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs;{74c5c6af-5114-4426-9216-20d81f8519ae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemWatcherErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = <IRemoteSystemWatcherErrorOccurredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemWatcherErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs";
}
::windows::core::interface_hierarchy!(RemoteSystemWatcherErrorOccurredEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteSystemWatcherErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for RemoteSystemWatcherErrorOccurredEventArgs {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemWebAccountFilter(::windows::core::IUnknown);
impl RemoteSystemWebAccountFilter {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Account(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Account)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn Create(account: &super::super::Security::Credentials::WebAccount) -> ::windows::core::Result<RemoteSystemWebAccountFilter> {
        Self::IRemoteSystemWebAccountFilterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(account), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteSystemWebAccountFilterFactory<R, F: FnOnce(&IRemoteSystemWebAccountFilterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteSystemWebAccountFilter, IRemoteSystemWebAccountFilterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteSystemWebAccountFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteSystemWebAccountFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteSystemWebAccountFilter {}
impl ::core::fmt::Debug for RemoteSystemWebAccountFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWebAccountFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemWebAccountFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteSystems.RemoteSystemWebAccountFilter;{3fb75873-87c8-5d8f-977e-f69f96d67238})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteSystemWebAccountFilter {
    const IID: ::windows::core::GUID = <IRemoteSystemWebAccountFilter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteSystemWebAccountFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWebAccountFilter";
}
::windows::core::interface_hierarchy!(RemoteSystemWebAccountFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: RemoteSystemWebAccountFilter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemWebAccountFilter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&RemoteSystemWebAccountFilter> for ::windows::core::InParam<IRemoteSystemFilter> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteSystemWebAccountFilter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RemoteSystemWebAccountFilter {}
unsafe impl ::core::marker::Sync for RemoteSystemWebAccountFilter {}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemAccessStatus(pub i32);
impl RemoteSystemAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemAccessStatus {}
impl ::core::clone::Clone for RemoteSystemAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemAuthorizationKind(pub i32);
impl RemoteSystemAuthorizationKind {
    pub const SameUser: Self = Self(0i32);
    pub const Anonymous: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemAuthorizationKind {}
impl ::core::clone::Clone for RemoteSystemAuthorizationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemAuthorizationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemAuthorizationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemAuthorizationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemAuthorizationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemAuthorizationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemAuthorizationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemDiscoveryType(pub i32);
impl RemoteSystemDiscoveryType {
    pub const Any: Self = Self(0i32);
    pub const Proximal: Self = Self(1i32);
    pub const Cloud: Self = Self(2i32);
    pub const SpatiallyProximal: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemDiscoveryType {}
impl ::core::clone::Clone for RemoteSystemDiscoveryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemDiscoveryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemDiscoveryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemDiscoveryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemDiscoveryType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemDiscoveryType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemDiscoveryType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemPlatform(pub i32);
impl RemoteSystemPlatform {
    pub const Unknown: Self = Self(0i32);
    pub const Windows: Self = Self(1i32);
    pub const Android: Self = Self(2i32);
    pub const Ios: Self = Self(3i32);
    pub const Linux: Self = Self(4i32);
}
impl ::core::marker::Copy for RemoteSystemPlatform {}
impl ::core::clone::Clone for RemoteSystemPlatform {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemPlatform {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemPlatform {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemPlatform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemPlatform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemPlatform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemPlatform;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemSessionCreationStatus(pub i32);
impl RemoteSystemSessionCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemSessionCreationStatus {}
impl ::core::clone::Clone for RemoteSystemSessionCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemSessionCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemSessionDisconnectedReason(pub i32);
impl RemoteSystemSessionDisconnectedReason {
    pub const SessionUnavailable: Self = Self(0i32);
    pub const RemovedByController: Self = Self(1i32);
    pub const SessionClosed: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemSessionDisconnectedReason {}
impl ::core::clone::Clone for RemoteSystemSessionDisconnectedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionDisconnectedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemSessionDisconnectedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionDisconnectedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionDisconnectedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionDisconnectedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemSessionJoinStatus(pub i32);
impl RemoteSystemSessionJoinStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
    pub const SessionUnavailable: Self = Self(3i32);
    pub const RejectedByController: Self = Self(4i32);
}
impl ::core::marker::Copy for RemoteSystemSessionJoinStatus {}
impl ::core::clone::Clone for RemoteSystemSessionJoinStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionJoinStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemSessionJoinStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionJoinStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionJoinStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionJoinStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionJoinStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemSessionMessageChannelReliability(pub i32);
impl RemoteSystemSessionMessageChannelReliability {
    pub const Reliable: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemSessionMessageChannelReliability {}
impl ::core::clone::Clone for RemoteSystemSessionMessageChannelReliability {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionMessageChannelReliability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemSessionMessageChannelReliability {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionMessageChannelReliability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionMessageChannelReliability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionMessageChannelReliability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannelReliability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemSessionParticipantWatcherStatus(pub i32);
impl RemoteSystemSessionParticipantWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for RemoteSystemSessionParticipantWatcherStatus {}
impl ::core::clone::Clone for RemoteSystemSessionParticipantWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionParticipantWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemSessionParticipantWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionParticipantWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionParticipantWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionParticipantWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemSessionWatcherStatus(pub i32);
impl RemoteSystemSessionWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for RemoteSystemSessionWatcherStatus {}
impl ::core::clone::Clone for RemoteSystemSessionWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemSessionWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemSessionWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemSessionWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemSessionWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemSessionWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemSessionWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemStatus(pub i32);
impl RemoteSystemStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const DiscoveringAvailability: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemStatus {}
impl ::core::clone::Clone for RemoteSystemStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemStatusType(pub i32);
impl RemoteSystemStatusType {
    pub const Any: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemStatusType {}
impl ::core::clone::Clone for RemoteSystemStatusType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemStatusType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemStatusType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemStatusType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemStatusType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemStatusType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemStatusType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RemoteSystemWatcherError(pub i32);
impl RemoteSystemWatcherError {
    pub const Unknown: Self = Self(0i32);
    pub const InternetNotAvailable: Self = Self(1i32);
    pub const AuthenticationError: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemWatcherError {}
impl ::core::clone::Clone for RemoteSystemWatcherError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RemoteSystemWatcherError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RemoteSystemWatcherError {
    type Abi = Self;
}
impl ::core::fmt::Debug for RemoteSystemWatcherError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteSystemWatcherError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteSystemWatcherError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.RemoteSystems.RemoteSystemWatcherError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
