#[cfg(feature = "Devices_Enumeration_Pnp")]
pub mod Pnp;
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDeviceAccessChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceAccessChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdeda0bcc_4f9d_4f58_9dba_a9bc800408d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccessStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessChangedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessChangedEventArgs2 {
    type Vtable = IDeviceAccessChangedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IDeviceAccessChangedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceAccessChangedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82523262_934b_4b30_a178_adc39f2f2be3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessChangedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_Vtbl;
}
impl ::core::clone::Clone for IDeviceAccessInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceAccessInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0baa9a73_6de5_4915_8ddd_9a0554a6f545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccessChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessChanged: usize,
    pub CurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccessStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccessInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceAccessInformationStatics {
    type Vtable = IDeviceAccessInformationStatics_Vtbl;
}
impl ::core::clone::Clone for IDeviceAccessInformationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceAccessInformationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x574bd3d3_5f30_45cd_8a94_724fe5973084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccessInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromDeviceClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclassid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceConnectionChangeTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IDeviceConnectionChangeTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceConnectionChangeTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8578c0c_bbc1_484b_bffa_7b31dcc200b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceDisconnectButtonClickedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDeviceDisconnectButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceDisconnectButtonClickedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e44b56d_f902_4a00_b536_f37992e6a2a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceDisconnectButtonClickedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformation {
    type Vtable = IDeviceInformation_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaba0fb95_4398_489d_8e44_e6130927011f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub EnclosureLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetGlyphThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetGlyphThumbnailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformation2 {
    type Vtable = IDeviceInformation2_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf156a638_7997_48d9_a10c_269d46533f48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceInformationKind) -> ::windows_core::HRESULT,
    pub Pairing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationCustomPairing(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationCustomPairing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationCustomPairing {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85138c02_4ee6_4914_8370_107a39144c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationCustomPairing_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAndSettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePairingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePairingRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairing(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationPairing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationPairing {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c4769f5_f684_40d5_8469_e8dbaab70485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsPaired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairing2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairing2 {
    type Vtable = IDeviceInformationPairing2_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationPairing2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationPairing2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf68612fd_0aee_4328_85cc_1c742bb1790d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairing2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingProtectionLevel) -> ::windows_core::HRESULT,
    pub Custom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PairWithProtectionLevelAndSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PairWithProtectionLevelAndSettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnpairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnpairAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairingStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairingStatics {
    type Vtable = IDeviceInformationPairingStatics_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationPairingStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationPairingStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe915c408_36d4_49a1_bf13_514173799b6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryRegisterForAllInboundPairingRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationPairingStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationPairingStatics2 {
    type Vtable = IDeviceInformationPairingStatics2_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationPairingStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationPairingStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04de5372_b7b7_476b_a74f_c5836a704d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationPairingStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TryRegisterForAllInboundPairingRequestsWithProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationStatics {
    type Vtable = IDeviceInformationStatics_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc17f100e_3a46_4a78_8013_769dc9b97390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncDeviceClass: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilter: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncAqsFilterAndAdditionalProperties: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWatcherDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWatcherAqsFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherAqsFilterAndAdditionalProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationStatics2 {
    type Vtable = IDeviceInformationStatics2_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x493b4f34_a84f_45fd_9167_15d1cb1bd1f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetAqsFilterFromDeviceClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIdAsyncWithKindAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, additionalproperties: *mut ::core::ffi::c_void, kind: DeviceInformationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIdAsyncWithKindAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsyncWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, additionalproperties: *mut ::core::ffi::c_void, kind: DeviceInformationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsyncWithKindAqsFilterAndAdditionalProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithKindAqsFilterAndAdditionalProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aqsfilter: ::std::mem::MaybeUninit<::windows_core::HSTRING>, additionalproperties: *mut ::core::ffi::c_void, kind: DeviceInformationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithKindAqsFilterAndAdditionalProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationUpdate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationUpdate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f315305_d972_44b7_a37e_9e822c78213b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceInformationUpdate2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceInformationUpdate2 {
    type Vtable = IDeviceInformationUpdate2_Vtbl;
}
impl ::core::clone::Clone for IDeviceInformationUpdate2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceInformationUpdate2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d9d148c_a873_485e_baa6_aa620788e3cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceInformationUpdate2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceInformationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePairingRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDevicePairingRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePairingRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf717fc56_de6b_487f_8376_0180aca69963);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PairingKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingKinds) -> ::windows_core::HRESULT,
    pub Pin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptWithPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePairingRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePairingRequestedEventArgs2 {
    type Vtable = IDevicePairingRequestedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IDevicePairingRequestedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePairingRequestedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc83752d9_e4d3_4db0_a360_a105e437dbdc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub AcceptWithPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passwordcredential: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    AcceptWithPasswordCredential: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePairingResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePairingResult {
    type Vtable = IDevicePairingResult_Vtbl;
}
impl ::core::clone::Clone for IDevicePairingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePairingResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x072b02bf_dd95_4025_9b37_de51adba37b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingResultStatus) -> ::windows_core::HRESULT,
    pub ProtectionLevelUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingProtectionLevel) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct IDevicePairingSettings(::windows_core::IUnknown);
impl IDevicePairingSettings {}
::windows_core::imp::interface_hierarchy!(IDevicePairingSettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IDevicePairingSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDevicePairingSettings {}
impl ::core::fmt::Debug for IDevicePairingSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDevicePairingSettings").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IDevicePairingSettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{482cb27c-83bb-420e-be51-6602b222de54}");
}
unsafe impl ::windows_core::Interface for IDevicePairingSettings {
    type Vtable = IDevicePairingSettings_Vtbl;
}
impl ::core::clone::Clone for IDevicePairingSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePairingSettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x482cb27c_83bb_420e_be51_6602b222de54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingSettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePicker {
    type Vtable = IDevicePicker_Vtbl;
}
impl ::core::clone::Clone for IDevicePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84997aa2_034a_4440_8813_7d0bd479bf5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestedProperties: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnectButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnectButtonClicked: usize,
    #[cfg(feature = "Foundation")]
    pub DevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPlacement: usize,
    #[cfg(feature = "Foundation")]
    pub PickSingleDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickSingleDeviceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub PickSingleDeviceAsyncWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    PickSingleDeviceAsyncWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDisplayStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, status: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: DevicePickerDisplayStatusOptions) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePickerAppearance(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_Vtbl;
}
impl ::core::clone::Clone for IDevicePickerAppearance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePickerAppearance {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe69a12c6_e627_4ed8_9b6c_460af445e56d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerAppearance_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    ForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    BackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub AccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    AccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetAccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedForegroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub SelectedAccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SelectedAccentColor: usize,
    #[cfg(feature = "UI")]
    pub SetSelectedAccentColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSelectedAccentColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDevicePickerFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDevicePickerFilter {
    type Vtable = IDevicePickerFilter_Vtbl;
}
impl ::core::clone::Clone for IDevicePickerFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDevicePickerFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91db92a2_57cb_48f1_9b59_a59b7a1f02a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePickerFilter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceClasses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceClasses: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedDeviceSelectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedDeviceSelectors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceSelectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceSelectedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x269edade_1d2f_4940_8402_4156b81d3c77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSelectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceUnpairingResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_Vtbl;
}
impl ::core::clone::Clone for IDeviceUnpairingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceUnpairingResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66f44ad3_79d9_444b_92cf_a92ef72571c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUnpairingResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceUnpairingResultStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcher {
    type Vtable = IDeviceWatcher_Vtbl;
}
impl ::core::clone::Clone for IDeviceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9eab97d_8f6b_4f96_a9f4_abc814e22271);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcher2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcher2 {
    type Vtable = IDeviceWatcher2_Vtbl;
}
impl ::core::clone::Clone for IDeviceWatcher2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceWatcher2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff08456e_ed14_49e9_9a69_8117c54ae971);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcher2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub GetBackgroundTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedeventkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections")))]
    GetBackgroundTrigger: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcherEvent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_Vtbl;
}
impl ::core::clone::Clone for IDeviceWatcherEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceWatcherEvent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74aa9c0b_1dbd_47fd_b635_3cc556d0ff8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherEvent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceWatcherEventKind) -> ::windows_core::HRESULT,
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DeviceInformationUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcherTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IDeviceWatcherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDeviceWatcherTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38808119_4cb7_4e57_a56d_776d07cbfef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub DeviceWatcherEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeviceWatcherEvents: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnclosureLocation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnclosureLocation {
    type Vtable = IEnclosureLocation_Vtbl;
}
impl ::core::clone::Clone for IEnclosureLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEnclosureLocation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42340a27_5810_459c_aabb_c65e1f813ecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InDock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub InLid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Panel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Panel) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnclosureLocation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEnclosureLocation2 {
    type Vtable = IEnclosureLocation2_Vtbl;
}
impl ::core::clone::Clone for IEnclosureLocation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IEnclosureLocation2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2885995b_e07d_485d_8a9e_bdf29aef4f66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnclosureLocation2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RotationAngleInDegreesClockwise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceAccessChangedEventArgs(::windows_core::IUnknown);
impl DeviceAccessChangedEventArgs {
    pub fn Status(&self) -> ::windows_core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IDeviceAccessChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceAccessChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccessChangedEventArgs {}
impl ::core::fmt::Debug for DeviceAccessChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceAccessChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessChangedEventArgs;{deda0bcc-4f9d-4f58-9dba-a9bc800408d5})");
}
impl ::core::clone::Clone for DeviceAccessChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceAccessChangedEventArgs {
    type Vtable = IDeviceAccessChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceAccessChangedEventArgs {
    const IID: ::windows_core::GUID = <IDeviceAccessChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DeviceAccessChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceAccessChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceAccessInformation(::windows_core::IUnknown);
impl DeviceAccessInformation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccessChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccessChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAccessChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn CurrentStatus(&self) -> ::windows_core::Result<DeviceAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromId(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromDeviceClassId(deviceclassid: ::windows_core::GUID) -> ::windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDeviceClassId)(::windows_core::Interface::as_raw(this), deviceclassid, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<DeviceAccessInformation> {
        Self::IDeviceAccessInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceAccessInformationStatics<R, F: FnOnce(&IDeviceAccessInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceAccessInformation, IDeviceAccessInformationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DeviceAccessInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccessInformation {}
impl ::core::fmt::Debug for DeviceAccessInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessInformation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceAccessInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceAccessInformation;{0baa9a73-6de5-4915-8ddd-9a0554a6f545})");
}
impl ::core::clone::Clone for DeviceAccessInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceAccessInformation {
    type Vtable = IDeviceAccessInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceAccessInformation {
    const IID: ::windows_core::GUID = <IDeviceAccessInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceAccessInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceAccessInformation";
}
::windows_core::imp::interface_hierarchy!(DeviceAccessInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceAccessInformation {}
unsafe impl ::core::marker::Sync for DeviceAccessInformation {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceConnectionChangeTriggerDetails(::windows_core::IUnknown);
impl DeviceConnectionChangeTriggerDetails {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceConnectionChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceConnectionChangeTriggerDetails {}
impl ::core::fmt::Debug for DeviceConnectionChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceConnectionChangeTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceConnectionChangeTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails;{b8578c0c-bbc1-484b-bffa-7b31dcc200b2})");
}
impl ::core::clone::Clone for DeviceConnectionChangeTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceConnectionChangeTriggerDetails {
    type Vtable = IDeviceConnectionChangeTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceConnectionChangeTriggerDetails {
    const IID: ::windows_core::GUID = <IDeviceConnectionChangeTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceConnectionChangeTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceConnectionChangeTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(DeviceConnectionChangeTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceConnectionChangeTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceConnectionChangeTriggerDetails {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceDisconnectButtonClickedEventArgs(::windows_core::IUnknown);
impl DeviceDisconnectButtonClickedEventArgs {
    pub fn Device(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Device)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceDisconnectButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceDisconnectButtonClickedEventArgs {}
impl ::core::fmt::Debug for DeviceDisconnectButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDisconnectButtonClickedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceDisconnectButtonClickedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs;{8e44b56d-f902-4a00-b536-f37992e6a2a7})");
}
impl ::core::clone::Clone for DeviceDisconnectButtonClickedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceDisconnectButtonClickedEventArgs {
    type Vtable = IDeviceDisconnectButtonClickedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceDisconnectButtonClickedEventArgs {
    const IID: ::windows_core::GUID = <IDeviceDisconnectButtonClickedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceDisconnectButtonClickedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DeviceDisconnectButtonClickedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceDisconnectButtonClickedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceDisconnectButtonClickedEventArgs {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceInformation(::windows_core::IUnknown);
impl DeviceInformation {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnclosureLocation(&self) -> ::windows_core::Result<EnclosureLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnclosureLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Update<P0>(&self, updateinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<DeviceInformationUpdate>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), updateinfo.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetGlyphThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetGlyphThumbnailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<DeviceInformationKind> {
        let this = &::windows_core::ComInterface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Pairing(&self) -> ::windows_core::Result<DeviceInformationPairing> {
        let this = &::windows_core::ComInterface::cast::<IDeviceInformation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pairing)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsyncAdditionalProperties<P0>(deviceid: &::windows_core::HSTRING, additionalproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdAsyncAdditionalProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), additionalproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncAqsFilter(aqsfilter: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncAqsFilter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncAqsFilterAndAdditionalProperties<P0>(aqsfilter: &::windows_core::HSTRING, additionalproperties: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), additionalproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWatcherDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWatcherAqsFilter(aqsfilter: &::windows_core::HSTRING) -> ::windows_core::Result<DeviceWatcher> {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherAqsFilter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherAqsFilterAndAdditionalProperties<P0>(aqsfilter: &::windows_core::HSTRING, additionalproperties: P0) -> ::windows_core::Result<DeviceWatcher>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), additionalproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAqsFilterFromDeviceClass(deviceclass: DeviceClass) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAqsFilterFromDeviceClass)(::windows_core::Interface::as_raw(this), deviceclass, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIdAsyncWithKindAndAdditionalProperties<P0>(deviceid: &::windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIdAsyncWithKindAndAdditionalProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), additionalproperties.try_into_param()?.abi(), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties<P0>(aqsfilter: &::windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsyncWithKindAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), additionalproperties.try_into_param()?.abi(), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithKindAqsFilterAndAdditionalProperties<P0>(aqsfilter: &::windows_core::HSTRING, additionalproperties: P0, kind: DeviceInformationKind) -> ::windows_core::Result<DeviceWatcher>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IDeviceInformationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWatcherWithKindAqsFilterAndAdditionalProperties)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(aqsfilter), additionalproperties.try_into_param()?.abi(), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceInformationStatics<R, F: FnOnce(&IDeviceInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceInformation, IDeviceInformationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDeviceInformationStatics2<R, F: FnOnce(&IDeviceInformationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceInformation, IDeviceInformationStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DeviceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformation {}
impl ::core::fmt::Debug for DeviceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})");
}
impl ::core::clone::Clone for DeviceInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceInformation {
    type Vtable = IDeviceInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceInformation {
    const IID: ::windows_core::GUID = <IDeviceInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformation";
}
::windows_core::imp::interface_hierarchy!(DeviceInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceInformation {}
unsafe impl ::core::marker::Sync for DeviceInformation {}
#[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct DeviceInformationCollection(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl DeviceInformationCollection {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<DeviceInformation>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<DeviceInformation>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<DeviceInformation>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<DeviceInformation>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for DeviceInformationCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for DeviceInformationCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for DeviceInformationCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Devices.Enumeration.DeviceInformation;{aba0fb95-4398-489d-8e44-e6130927011f})))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for DeviceInformationCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for DeviceInformationCollection {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<DeviceInformation>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for DeviceInformationCollection {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVectorView<DeviceInformation> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for DeviceInformationCollection {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &DeviceInformationCollection {
    type Item = DeviceInformation;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(DeviceInformationCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<DeviceInformation>> for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IVectorView<DeviceInformation>> for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DeviceInformationCollection {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceInformationCustomPairing(::windows_core::IUnknown);
impl DeviceInformationCustomPairing {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PairAsync(&self, pairingkindssupported: DevicePairingKinds) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairAsync)(::windows_core::Interface::as_raw(this), pairingkindssupported, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PairWithProtectionLevelAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAsync)(::windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PairWithProtectionLevelAndSettingsAsync<P0>(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>
    where
        P0: ::windows_core::TryIntoParam<IDevicePairingSettings>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAndSettingsAsync)(::windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, devicepairingsettings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PairingRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairingRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePairingRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePairingRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for DeviceInformationCustomPairing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationCustomPairing {}
impl ::core::fmt::Debug for DeviceInformationCustomPairing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationCustomPairing").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceInformationCustomPairing {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationCustomPairing;{85138c02-4ee6-4914-8370-107a39144c0e})");
}
impl ::core::clone::Clone for DeviceInformationCustomPairing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceInformationCustomPairing {
    type Vtable = IDeviceInformationCustomPairing_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceInformationCustomPairing {
    const IID: ::windows_core::GUID = <IDeviceInformationCustomPairing as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformationCustomPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationCustomPairing";
}
::windows_core::imp::interface_hierarchy!(DeviceInformationCustomPairing, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceInformationCustomPairing {}
unsafe impl ::core::marker::Sync for DeviceInformationCustomPairing {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceInformationPairing(::windows_core::IUnknown);
impl DeviceInformationPairing {
    pub fn IsPaired(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPaired)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanPair(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanPair)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PairAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PairWithProtectionLevelAsync(&self, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAsync)(::windows_core::Interface::as_raw(this), minprotectionlevel, &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionLevel(&self) -> ::windows_core::Result<DevicePairingProtectionLevel> {
        let this = &::windows_core::ComInterface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Custom(&self) -> ::windows_core::Result<DeviceInformationCustomPairing> {
        let this = &::windows_core::ComInterface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Custom)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PairWithProtectionLevelAndSettingsAsync<P0>(&self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>
    where
        P0: ::windows_core::TryIntoParam<IDevicePairingSettings>,
    {
        let this = &::windows_core::ComInterface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairWithProtectionLevelAndSettingsAsync)(::windows_core::Interface::as_raw(this), minprotectionlevel, devicepairingsettings.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnpairAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>> {
        let this = &::windows_core::ComInterface::cast::<IDeviceInformationPairing2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnpairAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TryRegisterForAllInboundPairingRequests(pairingkindssupported: DevicePairingKinds) -> ::windows_core::Result<bool> {
        Self::IDeviceInformationPairingStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRegisterForAllInboundPairingRequests)(::windows_core::Interface::as_raw(this), pairingkindssupported, &mut result__).from_abi(result__)
        })
    }
    pub fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows_core::Result<bool> {
        Self::IDeviceInformationPairingStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryRegisterForAllInboundPairingRequestsWithProtectionLevel)(::windows_core::Interface::as_raw(this), pairingkindssupported, minprotectionlevel, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceInformationPairingStatics<R, F: FnOnce(&IDeviceInformationPairingStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IDeviceInformationPairingStatics2<R, F: FnOnce(&IDeviceInformationPairingStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceInformationPairing, IDeviceInformationPairingStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DeviceInformationPairing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationPairing {}
impl ::core::fmt::Debug for DeviceInformationPairing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationPairing").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceInformationPairing {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationPairing;{2c4769f5-f684-40d5-8469-e8dbaab70485})");
}
impl ::core::clone::Clone for DeviceInformationPairing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceInformationPairing {
    type Vtable = IDeviceInformationPairing_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceInformationPairing {
    const IID: ::windows_core::GUID = <IDeviceInformationPairing as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformationPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationPairing";
}
::windows_core::imp::interface_hierarchy!(DeviceInformationPairing, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceInformationPairing {}
unsafe impl ::core::marker::Sync for DeviceInformationPairing {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceInformationUpdate(::windows_core::IUnknown);
impl DeviceInformationUpdate {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<DeviceInformationKind> {
        let this = &::windows_core::ComInterface::cast::<IDeviceInformationUpdate2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceInformationUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationUpdate {}
impl ::core::fmt::Debug for DeviceInformationUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationUpdate").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceInformationUpdate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceInformationUpdate;{8f315305-d972-44b7-a37e-9e822c78213b})");
}
impl ::core::clone::Clone for DeviceInformationUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceInformationUpdate {
    type Vtable = IDeviceInformationUpdate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceInformationUpdate {
    const IID: ::windows_core::GUID = <IDeviceInformationUpdate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceInformationUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceInformationUpdate";
}
::windows_core::imp::interface_hierarchy!(DeviceInformationUpdate, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceInformationUpdate {}
unsafe impl ::core::marker::Sync for DeviceInformationUpdate {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePairingRequestedEventArgs(::windows_core::IUnknown);
impl DevicePairingRequestedEventArgs {
    pub fn DeviceInformation(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PairingKind(&self) -> ::windows_core::Result<DevicePairingKinds> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PairingKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Pin(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pin)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Accept(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Accept)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptWithPin(&self, pin: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcceptWithPin)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(pin)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn AcceptWithPasswordCredential<P0>(&self, passwordcredential: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Security::Credentials::PasswordCredential>,
    {
        let this = &::windows_core::ComInterface::cast::<IDevicePairingRequestedEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AcceptWithPasswordCredential)(::windows_core::Interface::as_raw(this), passwordcredential.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for DevicePairingRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingRequestedEventArgs {}
impl ::core::fmt::Debug for DevicePairingRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePairingRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingRequestedEventArgs;{f717fc56-de6b-487f-8376-0180aca69963})");
}
impl ::core::clone::Clone for DevicePairingRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePairingRequestedEventArgs {
    type Vtable = IDevicePairingRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePairingRequestedEventArgs {
    const IID: ::windows_core::GUID = <IDevicePairingRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePairingRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DevicePairingRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePairingRequestedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePairingResult(::windows_core::IUnknown);
impl DevicePairingResult {
    pub fn Status(&self) -> ::windows_core::Result<DevicePairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionLevelUsed(&self) -> ::windows_core::Result<DevicePairingProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionLevelUsed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DevicePairingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingResult {}
impl ::core::fmt::Debug for DevicePairingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePairingResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePairingResult;{072b02bf-dd95-4025-9b37-de51adba37b7})");
}
impl ::core::clone::Clone for DevicePairingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePairingResult {
    type Vtable = IDevicePairingResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePairingResult {
    const IID: ::windows_core::GUID = <IDevicePairingResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePairingResult";
}
::windows_core::imp::interface_hierarchy!(DevicePairingResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePairingResult {}
unsafe impl ::core::marker::Sync for DevicePairingResult {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePicker(::windows_core::IUnknown);
impl DevicePicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DevicePicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Filter(&self) -> ::windows_core::Result<DevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Filter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Appearance(&self) -> ::windows_core::Result<DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Appearance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceSelected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceSelected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceSelected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDeviceSelected)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectButtonClicked<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisconnectButtonClicked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisconnectButtonClicked(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDisconnectButtonClicked)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DevicePickerDismissed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DevicePicker, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DevicePickerDismissed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDevicePickerDismissed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDevicePickerDismissed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Show(&self, selection: super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this), selection).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowWithPlacement(&self, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowWithPlacement)(::windows_core::Interface::as_raw(this), selection, placement).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickSingleDeviceAsync(&self, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleDeviceAsync)(::windows_core::Interface::as_raw(this), selection, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn PickSingleDeviceAsyncWithPlacement(&self, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickSingleDeviceAsyncWithPlacement)(::windows_core::Interface::as_raw(this), selection, placement, &mut result__).from_abi(result__)
        }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetDisplayStatus<P0>(&self, device: P0, status: &::windows_core::HSTRING, options: DevicePickerDisplayStatusOptions) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<DeviceInformation>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayStatus)(::windows_core::Interface::as_raw(this), device.into_param().abi(), ::core::mem::transmute_copy(status), options).ok() }
    }
}
impl ::core::cmp::PartialEq for DevicePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePicker {}
impl ::core::fmt::Debug for DevicePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePicker").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePicker;{84997aa2-034a-4440-8813-7d0bd479bf5a})");
}
impl ::core::clone::Clone for DevicePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePicker {
    type Vtable = IDevicePicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePicker {
    const IID: ::windows_core::GUID = <IDevicePicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePicker {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePicker";
}
::windows_core::imp::interface_hierarchy!(DevicePicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePicker {}
unsafe impl ::core::marker::Sync for DevicePicker {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePickerAppearance(::windows_core::IUnknown);
impl DevicePickerAppearance {
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn ForegroundColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForegroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetForegroundColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForegroundColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn BackgroundColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBackgroundColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackgroundColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn AccentColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccentColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetAccentColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAccentColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SelectedForegroundColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedForegroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetSelectedForegroundColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedForegroundColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SelectedBackgroundColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBackgroundColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetSelectedBackgroundColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedBackgroundColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SelectedAccentColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedAccentColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetSelectedAccentColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedAccentColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for DevicePickerAppearance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePickerAppearance {}
impl ::core::fmt::Debug for DevicePickerAppearance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerAppearance").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePickerAppearance {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerAppearance;{e69a12c6-e627-4ed8-9b6c-460af445e56d})");
}
impl ::core::clone::Clone for DevicePickerAppearance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePickerAppearance {
    type Vtable = IDevicePickerAppearance_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePickerAppearance {
    const IID: ::windows_core::GUID = <IDevicePickerAppearance as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePickerAppearance {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerAppearance";
}
::windows_core::imp::interface_hierarchy!(DevicePickerAppearance, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePickerAppearance {}
unsafe impl ::core::marker::Sync for DevicePickerAppearance {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DevicePickerFilter(::windows_core::IUnknown);
impl DevicePickerFilter {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDeviceClasses(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<DeviceClass>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedDeviceClasses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedDeviceSelectors(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedDeviceSelectors)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DevicePickerFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePickerFilter {}
impl ::core::fmt::Debug for DevicePickerFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerFilter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePickerFilter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DevicePickerFilter;{91db92a2-57cb-48f1-9b59-a59b7a1f02a2})");
}
impl ::core::clone::Clone for DevicePickerFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DevicePickerFilter {
    type Vtable = IDevicePickerFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePickerFilter {
    const IID: ::windows_core::GUID = <IDevicePickerFilter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePickerFilter {
    const NAME: &'static str = "Windows.Devices.Enumeration.DevicePickerFilter";
}
::windows_core::imp::interface_hierarchy!(DevicePickerFilter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DevicePickerFilter {}
unsafe impl ::core::marker::Sync for DevicePickerFilter {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceSelectedEventArgs(::windows_core::IUnknown);
impl DeviceSelectedEventArgs {
    pub fn SelectedDevice(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedDevice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceSelectedEventArgs {}
impl ::core::fmt::Debug for DeviceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceSelectedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceSelectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceSelectedEventArgs;{269edade-1d2f-4940-8402-4156b81d3c77})");
}
impl ::core::clone::Clone for DeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceSelectedEventArgs {
    type Vtable = IDeviceSelectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceSelectedEventArgs {
    const IID: ::windows_core::GUID = <IDeviceSelectedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceSelectedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DeviceSelectedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceSelectedEventArgs {}
#[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct DeviceThumbnail(::windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl DeviceThumbnail {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for DeviceThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for DeviceThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceThumbnail").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::RuntimeType for DeviceThumbnail {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceThumbnail;{cc254827-4b3d-438f-9232-10c76bc7e038})");
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for DeviceThumbnail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::Interface for DeviceThumbnail {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamWithContentType_Vtbl;
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::ComInterface for DeviceThumbnail {
    const IID: ::windows_core::GUID = <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::RuntimeName for DeviceThumbnail {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceThumbnail";
}
#[cfg(feature = "Storage_Streams")]
::windows_core::imp::interface_hierarchy!(DeviceThumbnail, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IContentTypeProvider> for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IInputStream> for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IOutputStream> for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStream> for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStreamWithContentType> for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for DeviceThumbnail {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceUnpairingResult(::windows_core::IUnknown);
impl DeviceUnpairingResult {
    pub fn Status(&self) -> ::windows_core::Result<DeviceUnpairingResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceUnpairingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceUnpairingResult {}
impl ::core::fmt::Debug for DeviceUnpairingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUnpairingResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceUnpairingResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceUnpairingResult;{66f44ad3-79d9-444b-92cf-a92ef72571c7})");
}
impl ::core::clone::Clone for DeviceUnpairingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceUnpairingResult {
    type Vtable = IDeviceUnpairingResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceUnpairingResult {
    const IID: ::windows_core::GUID = <IDeviceUnpairingResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceUnpairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceUnpairingResult";
}
::windows_core::imp::interface_hierarchy!(DeviceUnpairingResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceUnpairingResult {}
unsafe impl ::core::marker::Sync for DeviceUnpairingResult {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceWatcher(::windows_core::IUnknown);
impl DeviceWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Removed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<DeviceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections"))]
    pub fn GetBackgroundTrigger<P0>(&self, requestedeventkinds: P0) -> ::windows_core::Result<super::super::ApplicationModel::Background::DeviceWatcherTrigger>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind>>,
    {
        let this = &::windows_core::ComInterface::cast::<IDeviceWatcher2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetBackgroundTrigger)(::windows_core::Interface::as_raw(this), requestedeventkinds.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcher {}
impl ::core::fmt::Debug for DeviceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcher").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcher;{c9eab97d-8f6b-4f96-a9f4-abc814e22271})");
}
impl ::core::clone::Clone for DeviceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceWatcher {
    type Vtable = IDeviceWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceWatcher {
    const IID: ::windows_core::GUID = <IDeviceWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcher";
}
::windows_core::imp::interface_hierarchy!(DeviceWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceWatcher {}
unsafe impl ::core::marker::Sync for DeviceWatcher {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceWatcherEvent(::windows_core::IUnknown);
impl DeviceWatcherEvent {
    pub fn Kind(&self) -> ::windows_core::Result<DeviceWatcherEventKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceInformation(&self) -> ::windows_core::Result<DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceInformationUpdate(&self) -> ::windows_core::Result<DeviceInformationUpdate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationUpdate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceWatcherEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherEvent {}
impl ::core::fmt::Debug for DeviceWatcherEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherEvent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceWatcherEvent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherEvent;{74aa9c0b-1dbd-47fd-b635-3cc556d0ff8b})");
}
impl ::core::clone::Clone for DeviceWatcherEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceWatcherEvent {
    type Vtable = IDeviceWatcherEvent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceWatcherEvent {
    const IID: ::windows_core::GUID = <IDeviceWatcherEvent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceWatcherEvent {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherEvent";
}
::windows_core::imp::interface_hierarchy!(DeviceWatcherEvent, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceWatcherEvent {}
unsafe impl ::core::marker::Sync for DeviceWatcherEvent {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct DeviceWatcherTriggerDetails(::windows_core::IUnknown);
impl DeviceWatcherTriggerDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeviceWatcherEvents(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceWatcherEvents)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceWatcherTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherTriggerDetails {}
impl ::core::fmt::Debug for DeviceWatcherTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceWatcherTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.DeviceWatcherTriggerDetails;{38808119-4cb7-4e57-a56d-776d07cbfef9})");
}
impl ::core::clone::Clone for DeviceWatcherTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DeviceWatcherTriggerDetails {
    type Vtable = IDeviceWatcherTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceWatcherTriggerDetails {
    const IID: ::windows_core::GUID = <IDeviceWatcherTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.DeviceWatcherTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(DeviceWatcherTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DeviceWatcherTriggerDetails {}
unsafe impl ::core::marker::Sync for DeviceWatcherTriggerDetails {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
pub struct EnclosureLocation(::windows_core::IUnknown);
impl EnclosureLocation {
    pub fn InDock(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InDock)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InLid(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InLid)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Panel(&self) -> ::windows_core::Result<Panel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Panel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RotationAngleInDegreesClockwise(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IEnclosureLocation2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RotationAngleInDegreesClockwise)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EnclosureLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnclosureLocation {}
impl ::core::fmt::Debug for EnclosureLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnclosureLocation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EnclosureLocation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Enumeration.EnclosureLocation;{42340a27-5810-459c-aabb-c65e1f813ecf})");
}
impl ::core::clone::Clone for EnclosureLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for EnclosureLocation {
    type Vtable = IEnclosureLocation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EnclosureLocation {
    const IID: ::windows_core::GUID = <IEnclosureLocation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EnclosureLocation {
    const NAME: &'static str = "Windows.Devices.Enumeration.EnclosureLocation";
}
::windows_core::imp::interface_hierarchy!(EnclosureLocation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for EnclosureLocation {}
unsafe impl ::core::marker::Sync for EnclosureLocation {}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceAccessStatus(pub i32);
impl DeviceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceAccessStatus {}
impl ::core::clone::Clone for DeviceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeviceAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceAccessStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceClass(pub i32);
impl DeviceClass {
    pub const All: Self = Self(0i32);
    pub const AudioCapture: Self = Self(1i32);
    pub const AudioRender: Self = Self(2i32);
    pub const PortableStorageDevice: Self = Self(3i32);
    pub const VideoCapture: Self = Self(4i32);
    pub const ImageScanner: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceClass {}
impl ::core::clone::Clone for DeviceClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeviceClass {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceClass").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceClass {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceClass;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceInformationKind(pub i32);
impl DeviceInformationKind {
    pub const Unknown: Self = Self(0i32);
    pub const DeviceInterface: Self = Self(1i32);
    pub const DeviceContainer: Self = Self(2i32);
    pub const Device: Self = Self(3i32);
    pub const DeviceInterfaceClass: Self = Self(4i32);
    pub const AssociationEndpoint: Self = Self(5i32);
    pub const AssociationEndpointContainer: Self = Self(6i32);
    pub const AssociationEndpointService: Self = Self(7i32);
    pub const DevicePanel: Self = Self(8i32);
}
impl ::core::marker::Copy for DeviceInformationKind {}
impl ::core::clone::Clone for DeviceInformationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceInformationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeviceInformationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceInformationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceInformationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceInformationKind;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DevicePairingKinds(pub u32);
impl DevicePairingKinds {
    pub const None: Self = Self(0u32);
    pub const ConfirmOnly: Self = Self(1u32);
    pub const DisplayPin: Self = Self(2u32);
    pub const ProvidePin: Self = Self(4u32);
    pub const ConfirmPinMatch: Self = Self(8u32);
    pub const ProvidePasswordCredential: Self = Self(16u32);
}
impl ::core::marker::Copy for DevicePairingKinds {}
impl ::core::clone::Clone for DevicePairingKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePairingKinds {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DevicePairingKinds {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DevicePairingKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingKinds").field(&self.0).finish()
    }
}
impl DevicePairingKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DevicePairingKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DevicePairingKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePairingKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePairingKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DevicePairingKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for DevicePairingKinds {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingKinds;u4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DevicePairingProtectionLevel(pub i32);
impl DevicePairingProtectionLevel {
    pub const Default: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Encryption: Self = Self(2i32);
    pub const EncryptionAndAuthentication: Self = Self(3i32);
}
impl ::core::marker::Copy for DevicePairingProtectionLevel {}
impl ::core::clone::Clone for DevicePairingProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePairingProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DevicePairingProtectionLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DevicePairingProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingProtectionLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePairingProtectionLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingProtectionLevel;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DevicePairingResultStatus(pub i32);
impl DevicePairingResultStatus {
    pub const Paired: Self = Self(0i32);
    pub const NotReadyToPair: Self = Self(1i32);
    pub const NotPaired: Self = Self(2i32);
    pub const AlreadyPaired: Self = Self(3i32);
    pub const ConnectionRejected: Self = Self(4i32);
    pub const TooManyConnections: Self = Self(5i32);
    pub const HardwareFailure: Self = Self(6i32);
    pub const AuthenticationTimeout: Self = Self(7i32);
    pub const AuthenticationNotAllowed: Self = Self(8i32);
    pub const AuthenticationFailure: Self = Self(9i32);
    pub const NoSupportedProfiles: Self = Self(10i32);
    pub const ProtectionLevelCouldNotBeMet: Self = Self(11i32);
    pub const AccessDenied: Self = Self(12i32);
    pub const InvalidCeremonyData: Self = Self(13i32);
    pub const PairingCanceled: Self = Self(14i32);
    pub const OperationAlreadyInProgress: Self = Self(15i32);
    pub const RequiredHandlerNotRegistered: Self = Self(16i32);
    pub const RejectedByHandler: Self = Self(17i32);
    pub const RemoteDeviceHasAssociation: Self = Self(18i32);
    pub const Failed: Self = Self(19i32);
}
impl ::core::marker::Copy for DevicePairingResultStatus {}
impl ::core::clone::Clone for DevicePairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePairingResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DevicePairingResultStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DevicePairingResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingResultStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DevicePairingResultStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePairingResultStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DevicePickerDisplayStatusOptions(pub u32);
impl DevicePickerDisplayStatusOptions {
    pub const None: Self = Self(0u32);
    pub const ShowProgress: Self = Self(1u32);
    pub const ShowDisconnectButton: Self = Self(2u32);
    pub const ShowRetryButton: Self = Self(4u32);
}
impl ::core::marker::Copy for DevicePickerDisplayStatusOptions {}
impl ::core::clone::Clone for DevicePickerDisplayStatusOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DevicePickerDisplayStatusOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DevicePickerDisplayStatusOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DevicePickerDisplayStatusOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerDisplayStatusOptions").field(&self.0).finish()
    }
}
impl DevicePickerDisplayStatusOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePickerDisplayStatusOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePickerDisplayStatusOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for DevicePickerDisplayStatusOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DevicePickerDisplayStatusOptions;u4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceUnpairingResultStatus(pub i32);
impl DeviceUnpairingResultStatus {
    pub const Unpaired: Self = Self(0i32);
    pub const AlreadyUnpaired: Self = Self(1i32);
    pub const OperationAlreadyInProgress: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
}
impl ::core::marker::Copy for DeviceUnpairingResultStatus {}
impl ::core::clone::Clone for DeviceUnpairingResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceUnpairingResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeviceUnpairingResultStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceUnpairingResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUnpairingResultStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceUnpairingResultStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceUnpairingResultStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceWatcherEventKind(pub i32);
impl DeviceWatcherEventKind {
    pub const Add: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Remove: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceWatcherEventKind {}
impl ::core::clone::Clone for DeviceWatcherEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceWatcherEventKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeviceWatcherEventKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceWatcherEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherEventKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceWatcherEventKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherEventKind;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceWatcherStatus(pub i32);
impl DeviceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DeviceWatcherStatus {}
impl ::core::clone::Clone for DeviceWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DeviceWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.DeviceWatcherStatus;i4)");
}
#[doc = "*Required features: `\"Devices_Enumeration\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Panel(pub i32);
impl Panel {
    pub const Unknown: Self = Self(0i32);
    pub const Front: Self = Self(1i32);
    pub const Back: Self = Self(2i32);
    pub const Top: Self = Self(3i32);
    pub const Bottom: Self = Self(4i32);
    pub const Left: Self = Self(5i32);
    pub const Right: Self = Self(6i32);
}
impl ::core::marker::Copy for Panel {}
impl ::core::clone::Clone for Panel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Panel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for Panel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for Panel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Panel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for Panel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Enumeration.Panel;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
