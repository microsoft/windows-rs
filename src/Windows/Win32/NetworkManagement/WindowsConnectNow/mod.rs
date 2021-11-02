#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWCNConnectNotify(::windows::runtime::IUnknown);
impl IWCNConnectNotify {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn ConnectSucceeded(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn ConnectFailed(&self, hrfailure: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrfailure)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWCNConnectNotify {
    type Vtable = IWCNConnectNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3238051487, 54074, 19019, [191, 35, 187, 239, 70, 99, 208, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCNConnectNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrfailure: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWCNDevice(::windows::runtime::IUnknown);
impl IWCNDevice {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn SetPassword(&self, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(dwpasswordlength), ::std::mem::transmute(pbpassword)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, IWCNConnectNotify>>(&self, pnotify: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn GetAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(attributetype), ::std::mem::transmute(dwmaxbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwbufferused)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn GetIntegerAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(attributetype), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`, `Win32_Foundation`*"]
    pub unsafe fn GetStringAttribute(&self, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(attributetype), ::std::mem::transmute(cchmaxstring), ::std::mem::transmute(wszstring)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`, `Win32_Foundation`*"]
    pub unsafe fn GetNetworkProfile(&self, cchmaxstringlength: u32, wszprofile: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(cchmaxstringlength), ::std::mem::transmute(wszprofile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`, `Win32_Foundation`*"]
    pub unsafe fn SetNetworkProfile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszprofilexml: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszprofilexml.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn GetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvendorextspec), ::std::mem::transmute(dwmaxbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwbufferused)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn SetVendorExtension(&self, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvendorextspec), ::std::mem::transmute(cbbuffer), ::std::mem::transmute(pbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn Unadvise(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
    pub unsafe fn SetNFCPasswordParams(&self, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(dwoobpasswordid),
            ::std::mem::transmute(dwpasswordlength),
            ::std::mem::transmute(pbpassword),
            ::std::mem::transmute(dwremotepublickeyhashlength),
            ::std::mem::transmute(pbremotepublickeyhash),
            ::std::mem::transmute(dwdhkeybloblength),
            ::std::mem::transmute(pbdhkeyblob),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWCNDevice {
    type Vtable = IWCNDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3238051484, 54074, 19019, [191, 35, 187, 239, 70, 99, 208, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWCNDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: WCN_PASSWORD_TYPE, dwpasswordlength: u32, pbpassword: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnotify: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributetype: WCN_ATTRIBUTE_TYPE, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributetype: WCN_ATTRIBUTE_TYPE, puinteger: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributetype: WCN_ATTRIBUTE_TYPE, cchmaxstring: u32, wszstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchmaxstringlength: u32, wszprofile: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszprofilexml: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, dwmaxbuffersize: u32, pbbuffer: *mut u8, pdwbufferused: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvendorextspec: *const WCN_VENDOR_EXTENSION_SPEC, cbbuffer: u32, pbbuffer: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: WCN_PASSWORD_TYPE, dwoobpasswordid: u32, dwpasswordlength: u32, pbpassword: *const u8, dwremotepublickeyhashlength: u32, pbremotepublickeyhash: *const u8, dwdhkeybloblength: u32, pbdhkeyblob: *const u8) -> ::windows::runtime::HRESULT,
);
pub const SID_WcnProvider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3238051530, 54074, 19019, [191, 35, 187, 239, 70, 99, 208, 23]);
pub const WCNDeviceObject: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3238051495, 54074, 19019, [191, 35, 187, 239, 70, 99, 208, 23]);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_API_MAX_BUFFER_SIZE: u32 = 2096u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_ATTRIBUTE_TYPE(pub i32);
pub const WCN_TYPE_AP_CHANNEL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(0i32);
pub const WCN_TYPE_ASSOCIATION_STATE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(1i32);
pub const WCN_TYPE_AUTHENTICATION_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(2i32);
pub const WCN_TYPE_AUTHENTICATION_TYPE_FLAGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(3i32);
pub const WCN_TYPE_AUTHENTICATOR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(4i32);
pub const WCN_TYPE_CONFIG_METHODS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(5i32);
pub const WCN_TYPE_CONFIGURATION_ERROR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(6i32);
pub const WCN_TYPE_CONFIRMATION_URL4: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(7i32);
pub const WCN_TYPE_CONFIRMATION_URL6: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(8i32);
pub const WCN_TYPE_CONNECTION_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(9i32);
pub const WCN_TYPE_CONNECTION_TYPE_FLAGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(10i32);
pub const WCN_TYPE_CREDENTIAL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(11i32);
pub const WCN_TYPE_DEVICE_NAME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(12i32);
pub const WCN_TYPE_DEVICE_PASSWORD_ID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(13i32);
pub const WCN_TYPE_E_HASH1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(14i32);
pub const WCN_TYPE_E_HASH2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(15i32);
pub const WCN_TYPE_E_SNONCE1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(16i32);
pub const WCN_TYPE_E_SNONCE2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(17i32);
pub const WCN_TYPE_ENCRYPTED_SETTINGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(18i32);
pub const WCN_TYPE_ENCRYPTION_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(19i32);
pub const WCN_TYPE_ENCRYPTION_TYPE_FLAGS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(20i32);
pub const WCN_TYPE_ENROLLEE_NONCE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(21i32);
pub const WCN_TYPE_FEATURE_ID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(22i32);
pub const WCN_TYPE_IDENTITY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(23i32);
pub const WCN_TYPE_IDENTITY_PROOF: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(24i32);
pub const WCN_TYPE_KEY_WRAP_AUTHENTICATOR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(25i32);
pub const WCN_TYPE_KEY_IDENTIFIER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(26i32);
pub const WCN_TYPE_MAC_ADDRESS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(27i32);
pub const WCN_TYPE_MANUFACTURER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(28i32);
pub const WCN_TYPE_MESSAGE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(29i32);
pub const WCN_TYPE_MODEL_NAME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(30i32);
pub const WCN_TYPE_MODEL_NUMBER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(31i32);
pub const WCN_TYPE_NETWORK_INDEX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(32i32);
pub const WCN_TYPE_NETWORK_KEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(33i32);
pub const WCN_TYPE_NETWORK_KEY_INDEX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(34i32);
pub const WCN_TYPE_NEW_DEVICE_NAME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(35i32);
pub const WCN_TYPE_NEW_PASSWORD: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(36i32);
pub const WCN_TYPE_OOB_DEVICE_PASSWORD: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(37i32);
pub const WCN_TYPE_OS_VERSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(38i32);
pub const WCN_TYPE_POWER_LEVEL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(39i32);
pub const WCN_TYPE_PSK_CURRENT: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(40i32);
pub const WCN_TYPE_PSK_MAX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(41i32);
pub const WCN_TYPE_PUBLIC_KEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(42i32);
pub const WCN_TYPE_RADIO_ENABLED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(43i32);
pub const WCN_TYPE_REBOOT: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(44i32);
pub const WCN_TYPE_REGISTRAR_CURRENT: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(45i32);
pub const WCN_TYPE_REGISTRAR_ESTABLISHED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(46i32);
pub const WCN_TYPE_REGISTRAR_LIST: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(47i32);
pub const WCN_TYPE_REGISTRAR_MAX: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(48i32);
pub const WCN_TYPE_REGISTRAR_NONCE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(49i32);
pub const WCN_TYPE_REQUEST_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(50i32);
pub const WCN_TYPE_RESPONSE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(51i32);
pub const WCN_TYPE_RF_BANDS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(52i32);
pub const WCN_TYPE_R_HASH1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(53i32);
pub const WCN_TYPE_R_HASH2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(54i32);
pub const WCN_TYPE_R_SNONCE1: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(55i32);
pub const WCN_TYPE_R_SNONCE2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(56i32);
pub const WCN_TYPE_SELECTED_REGISTRAR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(57i32);
pub const WCN_TYPE_SERIAL_NUMBER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(58i32);
pub const WCN_TYPE_WI_FI_PROTECTED_SETUP_STATE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(59i32);
pub const WCN_TYPE_SSID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(60i32);
pub const WCN_TYPE_TOTAL_NETWORKS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(61i32);
pub const WCN_TYPE_UUID_E: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(62i32);
pub const WCN_TYPE_UUID_R: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(63i32);
pub const WCN_TYPE_VENDOR_EXTENSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(64i32);
pub const WCN_TYPE_VERSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(65i32);
pub const WCN_TYPE_X_509_CERTIFICATE_REQUEST: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(66i32);
pub const WCN_TYPE_X_509_CERTIFICATE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(67i32);
pub const WCN_TYPE_EAP_IDENTITY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(68i32);
pub const WCN_TYPE_MESSAGE_COUNTER: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(69i32);
pub const WCN_TYPE_PUBLIC_KEY_HASH: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(70i32);
pub const WCN_TYPE_REKEY_KEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(71i32);
pub const WCN_TYPE_KEY_LIFETIME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(72i32);
pub const WCN_TYPE_PERMITTED_CONFIG_METHODS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(73i32);
pub const WCN_TYPE_SELECTED_REGISTRAR_CONFIG_METHODS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(74i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(75i32);
pub const WCN_TYPE_SECONDARY_DEVICE_TYPE_LIST: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(76i32);
pub const WCN_TYPE_PORTABLE_DEVICE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(77i32);
pub const WCN_TYPE_AP_SETUP_LOCKED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(78i32);
pub const WCN_TYPE_APPLICATION_EXTENSION: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(79i32);
pub const WCN_TYPE_EAP_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(80i32);
pub const WCN_TYPE_INITIALIZATION_VECTOR: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(81i32);
pub const WCN_TYPE_KEY_PROVIDED_AUTOMATICALLY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(82i32);
pub const WCN_TYPE_802_1X_ENABLED: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(83i32);
pub const WCN_TYPE_APPSESSIONKEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(84i32);
pub const WCN_TYPE_WEPTRANSMITKEY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(85i32);
pub const WCN_TYPE_UUID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(86i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE_CATEGORY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(87i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE_SUBCATEGORY_OUI: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(88i32);
pub const WCN_TYPE_PRIMARY_DEVICE_TYPE_SUBCATEGORY: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(89i32);
pub const WCN_TYPE_CURRENT_SSID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(90i32);
pub const WCN_TYPE_BSSID: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(91i32);
pub const WCN_TYPE_DOT11_MAC_ADDRESS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(92i32);
pub const WCN_TYPE_AUTHORIZED_MACS: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(93i32);
pub const WCN_TYPE_NETWORK_KEY_SHAREABLE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(94i32);
pub const WCN_TYPE_REQUEST_TO_ENROLL: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(95i32);
pub const WCN_TYPE_REQUESTED_DEVICE_TYPE: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(96i32);
pub const WCN_TYPE_SETTINGS_DELAY_TIME: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(97i32);
pub const WCN_TYPE_VERSION2: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(98i32);
pub const WCN_TYPE_VENDOR_EXTENSION_WFA: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(99i32);
pub const WCN_NUM_ATTRIBUTE_TYPES: WCN_ATTRIBUTE_TYPE = WCN_ATTRIBUTE_TYPE(100i32);
impl ::std::convert::From<i32> for WCN_ATTRIBUTE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_ATTRIBUTE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_E_AUTHENTICATION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147206142i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_E_CONNECTION_REJECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147206141i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_E_PEER_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147206143i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_E_PROTOCOL_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147206139i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_E_SESSION_TIMEDOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147206140i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_FLAG_AUTHENTICATED_VE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_FLAG_DISCOVERY_VE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_FLAG_ENCRYPTED_VE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_MICROSOFT_VENDOR_ID: u32 = 311u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_NO_SUBTYPE: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_PASSWORD_TYPE(pub i32);
pub const WCN_PASSWORD_TYPE_PUSH_BUTTON: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(0i32);
pub const WCN_PASSWORD_TYPE_PIN: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(1i32);
pub const WCN_PASSWORD_TYPE_PIN_REGISTRAR_SPECIFIED: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(2i32);
pub const WCN_PASSWORD_TYPE_OOB_SPECIFIED: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(3i32);
pub const WCN_PASSWORD_TYPE_WFDS: WCN_PASSWORD_TYPE = WCN_PASSWORD_TYPE(4i32);
impl ::std::convert::From<i32> for WCN_PASSWORD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_PASSWORD_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_SESSION_STATUS(pub i32);
pub const WCN_SESSION_STATUS_SUCCESS: WCN_SESSION_STATUS = WCN_SESSION_STATUS(0i32);
pub const WCN_SESSION_STATUS_FAILURE_GENERIC: WCN_SESSION_STATUS = WCN_SESSION_STATUS(1i32);
pub const WCN_SESSION_STATUS_FAILURE_TIMEOUT: WCN_SESSION_STATUS = WCN_SESSION_STATUS(2i32);
impl ::std::convert::From<i32> for WCN_SESSION_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_SESSION_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_AUDIO_DEVICE: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_CAMERA: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_COMPUTER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_DISPLAY: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_GAMING_DEVICE: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_INPUT_DEVICE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_MULTIMEDIA_DEVICE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_NETWORK_INFRASTRUCTURE: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_OTHER: u32 = 255u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_PRINTER: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_STORAGE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_CATEGORY_TELEPHONE: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__HEADPHONES: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__HEADSET: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__HOMETHEATER: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__MICROPHONE: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__PMP: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__SPEAKERS: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_AUDIO_DEVICE__TUNER_RECEIVER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__SECURITY_CAMERA: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__STILL_CAMERA: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__VIDEO_CAMERA: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_CAMERA__WEB_CAMERA: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__DESKTOP: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__MEDIACENTER: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__MID: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__NETBOOK: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__NOTEBOOK: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__PC: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__SERVER: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_COMPUTER__ULTRAMOBILEPC: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__MONITOR: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__PICTURE_FRAME: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__PROJECTOR: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_DISPLAY__TELEVISION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__CONSOLE_ADAPT: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__PLAYSTATION: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__PORTABLE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__XBOX: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_GAMING_DEVICE__XBOX360: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__BARCODEREADER: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__BIOMETRICREADER: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__GAMECONTROLLER: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__JOYSTICK: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__KEYBOARD: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__MOUSE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__REMOTE: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__TOUCHSCREEN: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_INPUT_DEVICE__TRACKBALL: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__DAR: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__MCX: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__MEDIA_SERVER_ADAPT_EXT: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__PVP: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__PVR: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_MULTIMEDIA_DEVICE__SETTOPBOX: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__AP: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__BRIDGE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__GATEWAY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__ROUTER: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_NETWORK_INFRASTRUCUTURE__SWITCH: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__ALLINONE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__COPIER: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__FAX: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__PRINTER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_PRINTER__SCANNER: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_STORAGE__NAS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__PHONE_DUALMODE: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__PHONE_SINGLEMODE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__SMARTPHONE_DUALMODE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__SMARTPHONE_SINGLEMODE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_TELEPHONE__WINDOWS_MOBILE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub const WCN_VALUE_DT_SUBTYPE_WIFI_OUI: u32 = 5304836u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_ASSOCIATION_STATE(pub i32);
pub const WCN_VALUE_AS_NOT_ASSOCIATED: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(0i32);
pub const WCN_VALUE_AS_CONNECTION_SUCCESS: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(1i32);
pub const WCN_VALUE_AS_CONFIGURATION_FAILURE: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(2i32);
pub const WCN_VALUE_AS_ASSOCIATION_FAILURE: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(3i32);
pub const WCN_VALUE_AS_IP_FAILURE: WCN_VALUE_TYPE_ASSOCIATION_STATE = WCN_VALUE_TYPE_ASSOCIATION_STATE(4i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_ASSOCIATION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_ASSOCIATION_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_AUTHENTICATION_TYPE(pub i32);
pub const WCN_VALUE_AT_OPEN: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(1i32);
pub const WCN_VALUE_AT_WPAPSK: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(2i32);
pub const WCN_VALUE_AT_SHARED: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(4i32);
pub const WCN_VALUE_AT_WPA: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(8i32);
pub const WCN_VALUE_AT_WPA2: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(16i32);
pub const WCN_VALUE_AT_WPA2PSK: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(32i32);
pub const WCN_VALUE_AT_WPAWPA2PSK_MIXED: WCN_VALUE_TYPE_AUTHENTICATION_TYPE = WCN_VALUE_TYPE_AUTHENTICATION_TYPE(34i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_AUTHENTICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_AUTHENTICATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_BOOLEAN(pub i32);
pub const WCN_VALUE_FALSE: WCN_VALUE_TYPE_BOOLEAN = WCN_VALUE_TYPE_BOOLEAN(0i32);
pub const WCN_VALUE_TRUE: WCN_VALUE_TYPE_BOOLEAN = WCN_VALUE_TYPE_BOOLEAN(1i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_BOOLEAN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_BOOLEAN {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_CONFIGURATION_ERROR(pub i32);
pub const WCN_VALUE_CE_NO_ERROR: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(0i32);
pub const WCN_VALUE_CE_OOB_INTERFACE_READ_ERROR: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(1i32);
pub const WCN_VALUE_CE_DECRYPTION_CRC_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(2i32);
pub const WCN_VALUE_CE_2_4_CHANNEL_NOT_SUPPORTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(3i32);
pub const WCN_VALUE_CE_5_0_CHANNEL_NOT_SUPPORTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(4i32);
pub const WCN_VALUE_CE_SIGNAL_TOO_WEAK: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(5i32);
pub const WCN_VALUE_CE_NETWORK_AUTHENTICATION_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(6i32);
pub const WCN_VALUE_CE_NETWORK_ASSOCIATION_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(7i32);
pub const WCN_VALUE_CE_NO_DHCP_RESPONSE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(8i32);
pub const WCN_VALUE_CE_FAILED_DHCP_CONFIG: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(9i32);
pub const WCN_VALUE_CE_IP_ADDRESS_CONFLICT: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(10i32);
pub const WCN_VALUE_CE_COULD_NOT_CONNECT_TO_REGISTRAR: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(11i32);
pub const WCN_VALUE_CE_MULTIPLE_PBC_SESSIONS_DETECTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(12i32);
pub const WCN_VALUE_CE_ROGUE_ACTIVITY_SUSPECTED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(13i32);
pub const WCN_VALUE_CE_DEVICE_BUSY: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(14i32);
pub const WCN_VALUE_CE_SETUP_LOCKED: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(15i32);
pub const WCN_VALUE_CE_MESSAGE_TIMEOUT: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(16i32);
pub const WCN_VALUE_CE_REGISTRATION_SESSION_TIMEOUT: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(17i32);
pub const WCN_VALUE_CE_DEVICE_PASSWORD_AUTH_FAILURE: WCN_VALUE_TYPE_CONFIGURATION_ERROR = WCN_VALUE_TYPE_CONFIGURATION_ERROR(18i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_CONFIGURATION_ERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_CONFIGURATION_ERROR {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_CONFIG_METHODS(pub i32);
pub const WCN_VALUE_CM_USBA: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(1i32);
pub const WCN_VALUE_CM_ETHERNET: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(2i32);
pub const WCN_VALUE_CM_LABEL: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(4i32);
pub const WCN_VALUE_CM_DISPLAY: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(8i32);
pub const WCN_VALUE_CM_EXTERNAL_NFC: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(16i32);
pub const WCN_VALUE_CM_INTEGRATED_NFC: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(32i32);
pub const WCN_VALUE_CM_NFC_INTERFACE: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(64i32);
pub const WCN_VALUE_CM_PUSHBUTTON: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(128i32);
pub const WCN_VALUE_CM_KEYPAD: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(256i32);
pub const WCN_VALUE_CM_VIRT_PUSHBUTTON: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(640i32);
pub const WCN_VALUE_CM_PHYS_PUSHBUTTON: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(1152i32);
pub const WCN_VALUE_CM_VIRT_DISPLAY: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(8200i32);
pub const WCN_VALUE_CM_PHYS_DISPLAY: WCN_VALUE_TYPE_CONFIG_METHODS = WCN_VALUE_TYPE_CONFIG_METHODS(16392i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_CONFIG_METHODS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_CONFIG_METHODS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_CONNECTION_TYPE(pub i32);
pub const WCN_VALUE_CT_ESS: WCN_VALUE_TYPE_CONNECTION_TYPE = WCN_VALUE_TYPE_CONNECTION_TYPE(1i32);
pub const WCN_VALUE_CT_IBSS: WCN_VALUE_TYPE_CONNECTION_TYPE = WCN_VALUE_TYPE_CONNECTION_TYPE(2i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_CONNECTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_CONNECTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(pub i32);
pub const WCN_VALUE_DP_DEFAULT: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(0i32);
pub const WCN_VALUE_DP_USER_SPECIFIED: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(1i32);
pub const WCN_VALUE_DP_MACHINE_SPECIFIED: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(2i32);
pub const WCN_VALUE_DP_REKEY: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(3i32);
pub const WCN_VALUE_DP_PUSHBUTTON: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(4i32);
pub const WCN_VALUE_DP_REGISTRAR_SPECIFIED: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(5i32);
pub const WCN_VALUE_DP_NFC_CONNECTION_HANDOVER: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(7i32);
pub const WCN_VALUE_DP_WFD_SERVICES: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(8i32);
pub const WCN_VALUE_DP_OUTOFBAND_MIN: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(16i32);
pub const WCN_VALUE_DP_OUTOFBAND_MAX: WCN_VALUE_TYPE_DEVICE_PASSWORD_ID = WCN_VALUE_TYPE_DEVICE_PASSWORD_ID(65535i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_DEVICE_PASSWORD_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_DEVICE_PASSWORD_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_ENCRYPTION_TYPE(pub i32);
pub const WCN_VALUE_ET_NONE: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(1i32);
pub const WCN_VALUE_ET_WEP: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(2i32);
pub const WCN_VALUE_ET_TKIP: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(4i32);
pub const WCN_VALUE_ET_AES: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(8i32);
pub const WCN_VALUE_ET_TKIP_AES_MIXED: WCN_VALUE_TYPE_ENCRYPTION_TYPE = WCN_VALUE_TYPE_ENCRYPTION_TYPE(12i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_ENCRYPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_ENCRYPTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_MESSAGE_TYPE(pub i32);
pub const WCN_VALUE_MT_BEACON: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(1i32);
pub const WCN_VALUE_MT_PROBE_REQUEST: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(2i32);
pub const WCN_VALUE_MT_PROBE_RESPONSE: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(3i32);
pub const WCN_VALUE_MT_M1: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(4i32);
pub const WCN_VALUE_MT_M2: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(5i32);
pub const WCN_VALUE_MT_M2D: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(6i32);
pub const WCN_VALUE_MT_M3: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(7i32);
pub const WCN_VALUE_MT_M4: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(8i32);
pub const WCN_VALUE_MT_M5: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(9i32);
pub const WCN_VALUE_MT_M6: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(10i32);
pub const WCN_VALUE_MT_M7: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(11i32);
pub const WCN_VALUE_MT_M8: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(12i32);
pub const WCN_VALUE_MT_ACK: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(13i32);
pub const WCN_VALUE_MT_NACK: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(14i32);
pub const WCN_VALUE_MT_DONE: WCN_VALUE_TYPE_MESSAGE_TYPE = WCN_VALUE_TYPE_MESSAGE_TYPE(15i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_MESSAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_MESSAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub struct WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    pub Category: u16,
    pub SubCategoryOUI: u32,
    pub SubCategory: u16,
}
impl WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {}
impl ::std::default::Default for WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_PRIMARY_DEVICE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_REQUEST_TYPE(pub i32);
pub const WCN_VALUE_ReqT_ENROLLEE_INFO: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(0i32);
pub const WCN_VALUE_ReqT_ENROLLEE_OPEN_1X: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(1i32);
pub const WCN_VALUE_ReqT_REGISTRAR: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(2i32);
pub const WCN_VALUE_ReqT_MANAGER_REGISTRAR: WCN_VALUE_TYPE_REQUEST_TYPE = WCN_VALUE_TYPE_REQUEST_TYPE(3i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_REQUEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_REQUEST_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_RESPONSE_TYPE(pub i32);
pub const WCN_VALUE_RspT_ENROLLEE_INFO: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(0i32);
pub const WCN_VALUE_RspT_ENROLLEE_OPEN_1X: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(1i32);
pub const WCN_VALUE_RspT_REGISTRAR: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(2i32);
pub const WCN_VALUE_RspT_AP: WCN_VALUE_TYPE_RESPONSE_TYPE = WCN_VALUE_TYPE_RESPONSE_TYPE(3i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_RESPONSE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_RESPONSE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_RF_BANDS(pub i32);
pub const WCN_VALUE_RB_24GHZ: WCN_VALUE_TYPE_RF_BANDS = WCN_VALUE_TYPE_RF_BANDS(1i32);
pub const WCN_VALUE_RB_50GHZ: WCN_VALUE_TYPE_RF_BANDS = WCN_VALUE_TYPE_RF_BANDS(2i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_RF_BANDS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_RF_BANDS {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_VERSION(pub i32);
pub const WCN_VALUE_VERSION_1_0: WCN_VALUE_TYPE_VERSION = WCN_VALUE_TYPE_VERSION(16i32);
pub const WCN_VALUE_VERSION_2_0: WCN_VALUE_TYPE_VERSION = WCN_VALUE_TYPE_VERSION(32i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(pub i32);
pub const WCN_VALUE_SS_RESERVED00: WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE = WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(0i32);
pub const WCN_VALUE_SS_NOT_CONFIGURED: WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE = WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(1i32);
pub const WCN_VALUE_SS_CONFIGURED: WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE = WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE(2i32);
impl ::std::convert::From<i32> for WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCN_VALUE_TYPE_WI_FI_PROTECTED_SETUP_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_WindowsConnectNow`*"]
pub struct WCN_VENDOR_EXTENSION_SPEC {
    pub VendorId: u32,
    pub SubType: u32,
    pub Index: u32,
    pub Flags: u32,
}
impl WCN_VENDOR_EXTENSION_SPEC {}
impl ::std::default::Default for WCN_VENDOR_EXTENSION_SPEC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WCN_VENDOR_EXTENSION_SPEC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WCN_VENDOR_EXTENSION_SPEC").field("VendorId", &self.VendorId).field("SubType", &self.SubType).field("Index", &self.Index).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for WCN_VENDOR_EXTENSION_SPEC {
    fn eq(&self, other: &Self) -> bool {
        self.VendorId == other.VendorId && self.SubType == other.SubType && self.Index == other.Index && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for WCN_VENDOR_EXTENSION_SPEC {}
unsafe impl ::windows::runtime::Abi for WCN_VENDOR_EXTENSION_SPEC {
    type Abi = Self;
    type DefaultType = Self;
}
